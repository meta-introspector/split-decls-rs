// SRC: ../rust/compiler/rustc_middle/src/query/keys.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
// Defines the set of legal keys that can be used in queries.

use std::ffi::OsStr;

use crate::rustc_complete::def_id::{CrateNum, DefId, LOCAL_CRATE, LocalDefId, LocalModDefId, ModDefId};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::hir_id::{HirId, OwnerId};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use rustc_query_system::dep_graph::DepNodeIndex;
use rustc_query_system::query::{DefIdCache, DefaultCache, SingleCache, VecCache};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{DUMMY_SP, Ident, Span, Symbol};
/* AST_META: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */

use crate::infer::canonical::CanonicalQueryInput;
use crate::mir::mono::CollectionMode;
use crate::ty::fast_reject::SimplifiedType;
use crate::ty::layout::{TyAndLayout, ValidityRequirement};
/* AST_META: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::ty::{self, GenericArg, GenericArgsRef, Ty, TyCtxt};
/* AST_META: AST_ID=7 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::{mir, traits};
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=LocalCrate; | COMPLEXITY=19 | LINES=39 */

/// Placeholder for `CrateNum`'s "local" counterpart
#[derive(Copy, Clone, Debug)]
pub struct LocalCrate;

/// The `Key` trait controls what types can legally be used as the key
/// for a query.
pub trait Key: Sized {
    /// The type of in-memory cache to use for queries with this key type.
    ///
    /// In practice the cache type must implement [`QueryCache`], though that
    /// constraint is not enforced here.
    ///
    /// [`QueryCache`]: rustc_query_system::query::QueryCache
    // N.B. Most of the keys down below have `type Cache<V> = DefaultCache<Self, V>;`,
    //      it would be reasonable to use associated type defaults, to remove the duplication...
    //
    //      ...But r-a doesn't support them yet and using a default here causes r-a to not infer
    //      return types of queries which is very annoying. Thus, until r-a support associated
    //      type defaults, please restrain from using them here <3
    //
    //      r-a issue: <https://github.com/rust-lang/rust-analyzer/issues/13693>
    type Cache<V>;

    /// In the event that a cycle occurs, if no explicit span has been
    /// given for a query with key `self`, what span should we use?
    fn default_span(&self, tcx: TyCtxt<'_>) -> Span;

    /// If the key is a [`DefId`] or `DefId`--equivalent, return that `DefId`.
    /// Otherwise, return `None`.
    fn key_as_def_id(&self) -> Option<DefId> {
        None
    }

    /// Used to detect when ADT def ids are used as keys in a cycle for better error reporting.
    fn def_id_for_ty_in_cycle(&self) -> Option<DefId> {
        None
    }
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=as_local_key | COMPLEXITY=2 | LINES=8 */

pub trait AsLocalKey: Key {
    type LocalKey;

    /// Given an instance of this key, what crate is it referring to?
    /// This is used to find the provider.
    fn as_local_key(&self) -> Option<Self::LocalKey>;
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for () {
    type Cache<V> = SingleCache<V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for ty::InstanceKind<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.def_span(self.def_id())
    }
}
/* AST_META: AST_ID=12 | TYPE=FUNCTION | NAME=as_local_key | COMPLEXITY=5 | LINES=9 */

impl<'tcx> AsLocalKey for ty::InstanceKind<'tcx> {
    type LocalKey = Self;

    #[inline(always)]
    fn as_local_key(&self) -> Option<Self::LocalKey> {
        self.def_id().is_local().then(|| *self)
    }
}
/* AST_META: AST_ID=13 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for ty::Instance<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.def_span(self.def_id())
    }
}
/* AST_META: AST_ID=14 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for mir::interpret::GlobalId<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.instance.default_span(tcx)
    }
}
/* AST_META: AST_ID=15 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (Ty<'tcx>, Option<ty::ExistentialTraitRef<'tcx>>) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=16 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for mir::interpret::LitToConstInput<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _tcx: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=17 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for CrateNum {
    type Cache<V> = VecCache<Self, V, DepNodeIndex>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=18 | TYPE=FUNCTION | NAME=as_local_key | COMPLEXITY=5 | LINES=9 */

impl AsLocalKey for CrateNum {
    type LocalKey = LocalCrate;

    #[inline(always)]
    fn as_local_key(&self) -> Option<Self::LocalKey> {
        (*self == LOCAL_CRATE).then_some(LocalCrate)
    }
}
/* AST_META: AST_ID=19 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=6 | LINES=12 */

impl Key for OwnerId {
    type Cache<V> = VecCache<Self, V, DepNodeIndex>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.to_def_id().default_span(tcx)
    }

    fn key_as_def_id(&self) -> Option<DefId> {
        Some(self.to_def_id())
    }
}
/* AST_META: AST_ID=20 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=6 | LINES=12 */

impl Key for LocalDefId {
    type Cache<V> = VecCache<Self, V, DepNodeIndex>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.to_def_id().default_span(tcx)
    }

    fn key_as_def_id(&self) -> Option<DefId> {
        Some(self.to_def_id())
    }
}
/* AST_META: AST_ID=21 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=6 | LINES=13 */

impl Key for DefId {
    type Cache<V> = DefIdCache<V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.def_span(*self)
    }

    #[inline(always)]
    fn key_as_def_id(&self) -> Option<DefId> {
        Some(*self)
    }
}
/* AST_META: AST_ID=22 | TYPE=FUNCTION | NAME=as_local_key | COMPLEXITY=5 | LINES=9 */

impl AsLocalKey for DefId {
    type LocalKey = LocalDefId;

    #[inline(always)]
    fn as_local_key(&self) -> Option<Self::LocalKey> {
        self.as_local()
    }
}
/* AST_META: AST_ID=23 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=6 | LINES=13 */

impl Key for LocalModDefId {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.def_span(*self)
    }

    #[inline(always)]
    fn key_as_def_id(&self) -> Option<DefId> {
        Some(self.to_def_id())
    }
}
/* AST_META: AST_ID=24 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=6 | LINES=13 */

impl Key for ModDefId {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.def_span(*self)
    }

    #[inline(always)]
    fn key_as_def_id(&self) -> Option<DefId> {
        Some(self.to_def_id())
    }
}
/* AST_META: AST_ID=25 | TYPE=FUNCTION | NAME=as_local_key | COMPLEXITY=5 | LINES=9 */

impl AsLocalKey for ModDefId {
    type LocalKey = LocalModDefId;

    #[inline(always)]
    fn as_local_key(&self) -> Option<Self::LocalKey> {
        self.as_local()
    }
}
/* AST_META: AST_ID=26 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for SimplifiedType {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=27 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for (DefId, DefId) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.1.default_span(tcx)
    }
}
/* AST_META: AST_ID=28 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (ty::Instance<'tcx>, LocalDefId) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.0.default_span(tcx)
    }
}
/* AST_META: AST_ID=29 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for (DefId, LocalDefId) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.1.default_span(tcx)
    }
}
/* AST_META: AST_ID=30 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for (LocalDefId, DefId) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.0.default_span(tcx)
    }
}
/* AST_META: AST_ID=31 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for (LocalDefId, LocalDefId) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.0.default_span(tcx)
    }
}
/* AST_META: AST_ID=32 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=6 | LINES=13 */

impl Key for (DefId, Ident) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.def_span(self.0)
    }

    #[inline(always)]
    fn key_as_def_id(&self) -> Option<DefId> {
        Some(self.0)
    }
}
/* AST_META: AST_ID=33 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for (LocalDefId, LocalDefId, Ident) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.1.default_span(tcx)
    }
}
/* AST_META: AST_ID=34 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for (CrateNum, DefId) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.1.default_span(tcx)
    }
}
/* AST_META: AST_ID=35 | TYPE=FUNCTION | NAME=as_local_key | COMPLEXITY=5 | LINES=9 */

impl AsLocalKey for (CrateNum, DefId) {
    type LocalKey = DefId;

    #[inline(always)]
    fn as_local_key(&self) -> Option<Self::LocalKey> {
        (self.0 == LOCAL_CRATE).then(|| self.1)
    }
}
/* AST_META: AST_ID=36 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for (CrateNum, SimplifiedType) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=37 | TYPE=FUNCTION | NAME=as_local_key | COMPLEXITY=5 | LINES=9 */

impl AsLocalKey for (CrateNum, SimplifiedType) {
    type LocalKey = SimplifiedType;

    #[inline(always)]
    fn as_local_key(&self) -> Option<Self::LocalKey> {
        (self.0 == LOCAL_CRATE).then(|| self.1)
    }
}
/* AST_META: AST_ID=38 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for (DefId, SimplifiedType) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.0.default_span(tcx)
    }
}
/* AST_META: AST_ID=39 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for (DefId, ty::SizedTraitKind) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.0.default_span(tcx)
    }
}
/* AST_META: AST_ID=40 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for GenericArgsRef<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=41 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (DefId, GenericArgsRef<'tcx>) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.0.default_span(tcx)
    }
}
/* AST_META: AST_ID=42 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (ty::UnevaluatedConst<'tcx>, ty::UnevaluatedConst<'tcx>) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        (self.0).def.default_span(tcx)
    }
}
/* AST_META: AST_ID=43 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (LocalDefId, DefId, GenericArgsRef<'tcx>) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.0.default_span(tcx)
    }
}
/* AST_META: AST_ID=44 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (ty::ParamEnv<'tcx>, ty::TraitRef<'tcx>) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.def_span(self.1.def_id)
    }
}
/* AST_META: AST_ID=45 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for ty::ParamEnvAnd<'tcx, Ty<'tcx>> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _tcx: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=46 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for ty::TraitRef<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.def_span(self.def_id)
    }
}
/* AST_META: AST_ID=47 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for ty::PolyTraitRef<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.def_span(self.def_id())
    }
}
/* AST_META: AST_ID=48 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for ty::PolyExistentialTraitRef<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.def_span(self.def_id())
    }
}
/* AST_META: AST_ID=49 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (ty::PolyTraitRef<'tcx>, ty::PolyTraitRef<'tcx>) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.def_span(self.0.def_id())
    }
}
/* AST_META: AST_ID=50 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for GenericArg<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=51 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for ty::Const<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=52 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=10 | LINES=16 */

impl<'tcx> Key for Ty<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }

    fn def_id_for_ty_in_cycle(&self) -> Option<DefId> {
        match *self.kind() {
            ty::Adt(adt, _) => Some(adt.did()),
            ty::Coroutine(def_id, ..) => Some(def_id),
            _ => None,
        }
    }
}
/* AST_META: AST_ID=53 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for TyAndLayout<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=54 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (Ty<'tcx>, Ty<'tcx>) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=55 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for ty::Clauses<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=56 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for ty::ParamEnv<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=57 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=6 | LINES=12 */

impl<'tcx, T: Key> Key for ty::PseudoCanonicalInput<'tcx, T> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.value.default_span(tcx)
    }

    fn def_id_for_ty_in_cycle(&self) -> Option<DefId> {
        self.value.def_id_for_ty_in_cycle()
    }
}
/* AST_META: AST_ID=58 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for Symbol {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _tcx: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=59 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for Option<Symbol> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _tcx: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=60 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for &'tcx OsStr {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _tcx: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=61 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=10 */

/// Canonical query goals correspond to abstract trait operations that
/// are not tied to any crate in particular.
impl<'tcx, T: Clone> Key for CanonicalQueryInput<'tcx, T> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _tcx: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=62 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx, T: Clone> Key for (CanonicalQueryInput<'tcx, T>, bool) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _tcx: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=63 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl Key for (Symbol, u32, u32) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _tcx: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=64 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (DefId, Ty<'tcx>, GenericArgsRef<'tcx>, ty::ParamEnv<'tcx>) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _tcx: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=65 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (Ty<'tcx>, crate::rustc_abi::VariantIdx) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _tcx: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=66 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (ty::Predicate<'tcx>, traits::WellFormedLoc) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _tcx: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=67 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (ty::PolyFnSig<'tcx>, &'tcx ty::List<Ty<'tcx>>) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=68 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (ty::Instance<'tcx>, &'tcx ty::List<Ty<'tcx>>) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.0.default_span(tcx)
    }
}
/* AST_META: AST_ID=69 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for ty::Value<'tcx> {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }
}
/* AST_META: AST_ID=70 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=6 | LINES=13 */

impl Key for HirId {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.hir_span(*self)
    }

    #[inline(always)]
    fn key_as_def_id(&self) -> Option<DefId> {
        None
    }
}
/* AST_META: AST_ID=71 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=6 | LINES=13 */

impl Key for (LocalDefId, HirId) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        tcx.hir_span(self.1)
    }

    #[inline(always)]
    fn key_as_def_id(&self) -> Option<DefId> {
        Some(self.0.into())
    }
}
/* AST_META: AST_ID=72 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=10 | LINES=17 */

impl<'tcx> Key for (ValidityRequirement, ty::PseudoCanonicalInput<'tcx, Ty<'tcx>>) {
    type Cache<V> = DefaultCache<Self, V>;

    // Just forward to `Ty<'tcx>`

    fn default_span(&self, _: TyCtxt<'_>) -> Span {
        DUMMY_SP
    }

    fn def_id_for_ty_in_cycle(&self) -> Option<DefId> {
        match self.1.value.kind() {
            ty::Adt(adt, _) => Some(adt.did()),
            _ => None,
        }
    }
}
/* AST_META: AST_ID=73 | TYPE=FUNCTION | NAME=default_span | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Key for (ty::Instance<'tcx>, CollectionMode) {
    type Cache<V> = DefaultCache<Self, V>;

    fn default_span(&self, tcx: TyCtxt<'_>) -> Span {
        self.0.default_span(tcx)
    }
}