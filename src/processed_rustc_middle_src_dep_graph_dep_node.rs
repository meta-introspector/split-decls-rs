// SRC: ../rust/compiler/rustc_middle/src/dep_graph/dep_node.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_data_structures::fingerprint::Fingerprint;
use crate::rustc_complete::def_id::{CrateNum, DefId, LOCAL_CRATE, LocalDefId, LocalModDefId, ModDefId};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::definitions::DefPathHash;
use crate::rustc_complete::{HirId, ItemLocalId, OwnerId};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
pub use rustc_query_system::dep_graph::DepNode;
use rustc_query_system::dep_graph::FingerprintStyle;
pub use rustc_query_system::dep_graph::dep_node::DepKind;
pub(crate) use rustc_query_system::dep_graph::{DepContext, DepNodeParams};
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=47 | LINES=79 */
use crate::rustc_complete::Symbol;

use crate::mir::mono::MonoItem;
use crate::ty::TyCtxt;

macro_rules! define_dep_nodes {
    (
        $(
            $(#[$attr:meta])*
            [$($modifiers:tt)*] fn $variant:ident($($K:tt)*) -> $V:ty,
        )*
    ) => {

        #[macro_export]
        macro_rules! make_dep_kind_array {
            ($mod:ident) => {[ $($mod::$variant()),* ]};
        }

        #[macro_export]
        macro_rules! make_dep_kind_name_array {
            ($mod:ident) => {
                vec! {
                    $(*$mod::$variant().name),*
                }
            };
        }

        /// This enum serves as an index into arrays built by `make_dep_kind_array`.
        // This enum has more than u8::MAX variants so we need some kind of multi-byte
        // encoding. The derived Encodable/Decodable uses leb128 encoding which is
        // dense when only considering this enum. But DepKind is encoded in a larger
        // struct, and there we can take advantage of the unused bits in the u16.
        #[allow(non_camel_case_types)]
        #[repr(u16)] // Must be kept in sync with the inner type of `DepKind`.
        enum DepKindDefs {
            $( $( #[$attr] )* $variant),*
        }

        #[allow(non_upper_case_globals)]
        pub mod dep_kinds {
            use super::*;

            $(
                // The `as u16` cast must be kept in sync with the inner type of `DepKind`.
                pub const $variant: DepKind = DepKind::new(DepKindDefs::$variant as u16);
            )*
        }

        // This checks that the discriminants of the variants have been assigned consecutively
        // from 0 so that they can be used as a dense index.
        pub(crate) const DEP_KIND_VARIANTS: u16 = {
            let deps = &[$(dep_kinds::$variant,)*];
            let mut i = 0;
            while i < deps.len() {
                if i != deps[i].as_usize() {
                    panic!();
                }
                i += 1;
            }
            deps.len() as u16
        };

        pub(super) fn dep_kind_from_label_string(label: &str) -> Result<DepKind, ()> {
            match label {
                $(stringify!($variant) => Ok(dep_kinds::$variant),)*
                _ => Err(()),
            }
        }

        /// Contains variant => str representations for constructing
        /// DepNode groups for tests.
        #[allow(dead_code, non_upper_case_globals)]
        pub mod label_strs {
           $(
                pub const $variant: &str = stringify!($variant);
            )*
        }
    };
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=9 | LINES=21 */

// Create various data structures for each query, and also for a few things
// that aren't queries.
rustc_with_all_queries!(define_dep_nodes![
    /// We use this for most things when incr. comp. is turned off.
    [] fn Null() -> (),
    /// We use this to create a forever-red node.
    [] fn Red() -> (),
    [] fn SideEffect() -> (),
    [] fn AnonZeroDeps() -> (),
    [] fn TraitSelect() -> (),
    [] fn CompileCodegenUnit() -> (),
    [] fn CompileMonoItem() -> (),
    [] fn Metadata() -> (),
]);

// WARNING: `construct` is generic and does not know that `CompileCodegenUnit` takes `Symbol`s as keys.
// Be very careful changing this type signature!
pub(crate) fn make_compile_codegen_unit(tcx: TyCtxt<'_>, name: Symbol) -> DepNode {
    DepNode::construct(tcx, dep_kinds::CompileCodegenUnit, &name)
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

// WARNING: `construct` is generic and does not know that `CompileMonoItem` takes `MonoItem`s as keys.
// Be very careful changing this type signature!
pub(crate) fn make_compile_mono_item<'tcx>(
    tcx: TyCtxt<'tcx>,
    mono_item: &MonoItem<'tcx>,
) -> DepNode {
    DepNode::construct(tcx, dep_kinds::CompileMonoItem, mono_item)
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

// WARNING: `construct` is generic and does not know that `Metadata` takes `()`s as keys.
// Be very careful changing this type signature!
pub(crate) fn make_metadata(tcx: TyCtxt<'_>) -> DepNode {
    DepNode::construct(tcx, dep_kinds::Metadata, &())
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=extract_def_id | COMPLEXITY=2 | LINES=12 */

pub trait DepNodeExt: Sized {
    fn extract_def_id(&self, tcx: TyCtxt<'_>) -> Option<DefId>;

    fn from_label_string(
        tcx: TyCtxt<'_>,
        label: &str,
        def_path_hash: DefPathHash,
    ) -> Result<Self, ()>;

    fn has_label_string(label: &str) -> bool;
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=extract_def_id | COMPLEXITY=22 | LINES=42 */

impl DepNodeExt for DepNode {
    /// Extracts the DefId corresponding to this DepNode. This will work
    /// if two conditions are met:
    ///
    /// 1. The Fingerprint of the DepNode actually is a DefPathHash, and
    /// 2. the item that the DefPath refers to exists in the current tcx.
    ///
    /// Condition (1) is determined by the DepKind variant of the
    /// DepNode. Condition (2) might not be fulfilled if a DepNode
    /// refers to something from the previous compilation session that
    /// has been removed.
    fn extract_def_id(&self, tcx: TyCtxt<'_>) -> Option<DefId> {
        if tcx.fingerprint_style(self.kind) == FingerprintStyle::DefPathHash {
            tcx.def_path_hash_to_def_id(DefPathHash(self.hash.into()))
        } else {
            None
        }
    }

    /// Used in testing
    fn from_label_string(
        tcx: TyCtxt<'_>,
        label: &str,
        def_path_hash: DefPathHash,
    ) -> Result<DepNode, ()> {
        let kind = dep_kind_from_label_string(label)?;

        match tcx.fingerprint_style(kind) {
            FingerprintStyle::Opaque | FingerprintStyle::HirId => Err(()),
            FingerprintStyle::Unit => Ok(DepNode::new_no_params(tcx, kind)),
            FingerprintStyle::DefPathHash => {
                Ok(DepNode::from_def_path_hash(tcx, def_path_hash, kind))
            }
        }
    }

    /// Used in testing
    fn has_label_string(label: &str) -> bool {
        dep_kind_from_label_string(label).is_ok()
    }
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=fingerprint_style | COMPLEXITY=7 | LINES=17 */

impl<'tcx> DepNodeParams<TyCtxt<'tcx>> for () {
    #[inline(always)]
    fn fingerprint_style() -> FingerprintStyle {
        FingerprintStyle::Unit
    }

    #[inline(always)]
    fn to_fingerprint(&self, _: TyCtxt<'tcx>) -> Fingerprint {
        Fingerprint::ZERO
    }

    #[inline(always)]
    fn recover(_: TyCtxt<'tcx>, _: &DepNode) -> Option<Self> {
        Some(())
    }
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=fingerprint_style | COMPLEXITY=9 | LINES=22 */

impl<'tcx> DepNodeParams<TyCtxt<'tcx>> for DefId {
    #[inline(always)]
    fn fingerprint_style() -> FingerprintStyle {
        FingerprintStyle::DefPathHash
    }

    #[inline(always)]
    fn to_fingerprint(&self, tcx: TyCtxt<'tcx>) -> Fingerprint {
        tcx.def_path_hash(*self).0
    }

    #[inline(always)]
    fn to_debug_str(&self, tcx: TyCtxt<'tcx>) -> String {
        tcx.def_path_str(*self)
    }

    #[inline(always)]
    fn recover(tcx: TyCtxt<'tcx>, dep_node: &DepNode) -> Option<Self> {
        dep_node.extract_def_id(tcx)
    }
}
/* AST_META: AST_ID=12 | TYPE=FUNCTION | NAME=fingerprint_style | COMPLEXITY=9 | LINES=22 */

impl<'tcx> DepNodeParams<TyCtxt<'tcx>> for LocalDefId {
    #[inline(always)]
    fn fingerprint_style() -> FingerprintStyle {
        FingerprintStyle::DefPathHash
    }

    #[inline(always)]
    fn to_fingerprint(&self, tcx: TyCtxt<'tcx>) -> Fingerprint {
        self.to_def_id().to_fingerprint(tcx)
    }

    #[inline(always)]
    fn to_debug_str(&self, tcx: TyCtxt<'tcx>) -> String {
        self.to_def_id().to_debug_str(tcx)
    }

    #[inline(always)]
    fn recover(tcx: TyCtxt<'tcx>, dep_node: &DepNode) -> Option<Self> {
        dep_node.extract_def_id(tcx).map(|id| id.expect_local())
    }
}
/* AST_META: AST_ID=13 | TYPE=FUNCTION | NAME=fingerprint_style | COMPLEXITY=10 | LINES=22 */

impl<'tcx> DepNodeParams<TyCtxt<'tcx>> for OwnerId {
    #[inline(always)]
    fn fingerprint_style() -> FingerprintStyle {
        FingerprintStyle::DefPathHash
    }

    #[inline(always)]
    fn to_fingerprint(&self, tcx: TyCtxt<'tcx>) -> Fingerprint {
        self.to_def_id().to_fingerprint(tcx)
    }

    #[inline(always)]
    fn to_debug_str(&self, tcx: TyCtxt<'tcx>) -> String {
        self.to_def_id().to_debug_str(tcx)
    }

    #[inline(always)]
    fn recover(tcx: TyCtxt<'tcx>, dep_node: &DepNode) -> Option<Self> {
        dep_node.extract_def_id(tcx).map(|id| OwnerId { def_id: id.expect_local() })
    }
}
/* AST_META: AST_ID=14 | TYPE=FUNCTION | NAME=fingerprint_style | COMPLEXITY=9 | LINES=23 */

impl<'tcx> DepNodeParams<TyCtxt<'tcx>> for CrateNum {
    #[inline(always)]
    fn fingerprint_style() -> FingerprintStyle {
        FingerprintStyle::DefPathHash
    }

    #[inline(always)]
    fn to_fingerprint(&self, tcx: TyCtxt<'tcx>) -> Fingerprint {
        let def_id = self.as_def_id();
        def_id.to_fingerprint(tcx)
    }

    #[inline(always)]
    fn to_debug_str(&self, tcx: TyCtxt<'tcx>) -> String {
        tcx.crate_name(*self).to_string()
    }

    #[inline(always)]
    fn recover(tcx: TyCtxt<'tcx>, dep_node: &DepNode) -> Option<Self> {
        dep_node.extract_def_id(tcx).map(|id| id.krate)
    }
}
/* AST_META: AST_ID=15 | TYPE=FUNCTION | NAME=fingerprint_style | COMPLEXITY=10 | LINES=27 */

impl<'tcx> DepNodeParams<TyCtxt<'tcx>> for (DefId, DefId) {
    #[inline(always)]
    fn fingerprint_style() -> FingerprintStyle {
        FingerprintStyle::Opaque
    }

    // We actually would not need to specialize the implementation of this
    // method but it's faster to combine the hashes than to instantiate a full
    // hashing context and stable-hashing state.
    #[inline(always)]
    fn to_fingerprint(&self, tcx: TyCtxt<'tcx>) -> Fingerprint {
        let (def_id_0, def_id_1) = *self;

        let def_path_hash_0 = tcx.def_path_hash(def_id_0);
        let def_path_hash_1 = tcx.def_path_hash(def_id_1);

        def_path_hash_0.0.combine(def_path_hash_1.0)
    }

    #[inline(always)]
    fn to_debug_str(&self, tcx: TyCtxt<'tcx>) -> String {
        let (def_id_0, def_id_1) = *self;

        format!("({}, {})", tcx.def_path_debug_str(def_id_0), tcx.def_path_debug_str(def_id_1))
    }
}
/* AST_META: AST_ID=16 | TYPE=FUNCTION | NAME=fingerprint_style | COMPLEXITY=22 | LINES=43 */

impl<'tcx> DepNodeParams<TyCtxt<'tcx>> for HirId {
    #[inline(always)]
    fn fingerprint_style() -> FingerprintStyle {
        FingerprintStyle::HirId
    }

    // We actually would not need to specialize the implementation of this
    // method but it's faster to combine the hashes than to instantiate a full
    // hashing context and stable-hashing state.
    #[inline(always)]
    fn to_fingerprint(&self, tcx: TyCtxt<'tcx>) -> Fingerprint {
        let HirId { owner, local_id } = *self;
        let def_path_hash = tcx.def_path_hash(owner.to_def_id());
        Fingerprint::new(
            // `owner` is local, so is completely defined by the local hash
            def_path_hash.local_hash(),
            local_id.as_u32() as u64,
        )
    }

    #[inline(always)]
    fn to_debug_str(&self, tcx: TyCtxt<'tcx>) -> String {
        let HirId { owner, local_id } = *self;
        format!("{}.{}", tcx.def_path_str(owner), local_id.as_u32())
    }

    #[inline(always)]
    fn recover(tcx: TyCtxt<'tcx>, dep_node: &DepNode) -> Option<Self> {
        if tcx.fingerprint_style(dep_node.kind) == FingerprintStyle::HirId {
            let (local_hash, local_id) = Fingerprint::from(dep_node.hash).split();
            let def_path_hash = DefPathHash::new(tcx.stable_crate_id(LOCAL_CRATE), local_hash);
            let def_id = tcx.def_path_hash_to_def_id(def_path_hash)?.expect_local();
            let local_id = local_id
                .as_u64()
                .try_into()
                .unwrap_or_else(|_| panic!("local id should be u32, found {local_id:?}"));
            Some(HirId { owner: OwnerId { def_id }, local_id: ItemLocalId::from_u32(local_id) })
        } else {
            None
        }
    }
}
/* AST_META: AST_ID=17 | TYPE=FUNCTION | NAME=fingerprint_style | COMPLEXITY=9 | LINES=22 */

impl<'tcx> DepNodeParams<TyCtxt<'tcx>> for ModDefId {
    #[inline(always)]
    fn fingerprint_style() -> FingerprintStyle {
        FingerprintStyle::DefPathHash
    }

    #[inline(always)]
    fn to_fingerprint(&self, tcx: TyCtxt<'tcx>) -> Fingerprint {
        self.to_def_id().to_fingerprint(tcx)
    }

    #[inline(always)]
    fn to_debug_str(&self, tcx: TyCtxt<'tcx>) -> String {
        self.to_def_id().to_debug_str(tcx)
    }

    #[inline(always)]
    fn recover(tcx: TyCtxt<'tcx>, dep_node: &DepNode) -> Option<Self> {
        DefId::recover(tcx, dep_node).map(ModDefId::new_unchecked)
    }
}
/* AST_META: AST_ID=18 | TYPE=FUNCTION | NAME=fingerprint_style | COMPLEXITY=9 | LINES=22 */

impl<'tcx> DepNodeParams<TyCtxt<'tcx>> for LocalModDefId {
    #[inline(always)]
    fn fingerprint_style() -> FingerprintStyle {
        FingerprintStyle::DefPathHash
    }

    #[inline(always)]
    fn to_fingerprint(&self, tcx: TyCtxt<'tcx>) -> Fingerprint {
        self.to_def_id().to_fingerprint(tcx)
    }

    #[inline(always)]
    fn to_debug_str(&self, tcx: TyCtxt<'tcx>) -> String {
        self.to_def_id().to_debug_str(tcx)
    }

    #[inline(always)]
    fn recover(tcx: TyCtxt<'tcx>, dep_node: &DepNode) -> Option<Self> {
        LocalDefId::recover(tcx, dep_node).map(LocalModDefId::new_unchecked)
    }
}