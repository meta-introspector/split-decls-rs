// SRC: ../rust/compiler/rustc_metadata/src/rmeta/parameterized.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
use std::hash::Hash;

use crate::rustc_data_structures::unord::UnordMap;
use crate::rustc_complete::def_id::DefIndex;
use crate::rustc_index::{Idx, IndexVec};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::ty::{Binder, EarlyBinder};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_complete::Symbol;

use crate::rmeta::{LazyArray, LazyValue};
/* AST_META: AST_ID=4 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */

pub(crate) trait ParameterizedOverTcx: 'static {
    type Value<'tcx>;
}
/* AST_META: AST_ID=5 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4 */

impl<T: ParameterizedOverTcx> ParameterizedOverTcx for Option<T> {
    type Value<'tcx> = Option<T::Value<'tcx>>;
}
/* AST_META: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4 */

impl<A: ParameterizedOverTcx, B: ParameterizedOverTcx> ParameterizedOverTcx for (A, B) {
    type Value<'tcx> = (A::Value<'tcx>, B::Value<'tcx>);
}
/* AST_META: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4 */

impl<T: ParameterizedOverTcx> ParameterizedOverTcx for Vec<T> {
    type Value<'tcx> = Vec<T::Value<'tcx>>;
}
/* AST_META: AST_ID=8 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4 */

impl<I: Idx + 'static, T: ParameterizedOverTcx> ParameterizedOverTcx for IndexVec<I, T> {
    type Value<'tcx> = IndexVec<I, T::Value<'tcx>>;
}
/* AST_META: AST_ID=9 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4 */

impl<I: Hash + Eq + 'static, T: ParameterizedOverTcx> ParameterizedOverTcx for UnordMap<I, T> {
    type Value<'tcx> = UnordMap<I, T::Value<'tcx>>;
}
/* AST_META: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4 */

impl<T: ParameterizedOverTcx> ParameterizedOverTcx for Binder<'static, T> {
    type Value<'tcx> = Binder<'tcx, T::Value<'tcx>>;
}
/* AST_META: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4 */

impl<T: ParameterizedOverTcx> ParameterizedOverTcx for EarlyBinder<'static, T> {
    type Value<'tcx> = EarlyBinder<'tcx, T::Value<'tcx>>;
}
/* AST_META: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4 */

impl<T: ParameterizedOverTcx> ParameterizedOverTcx for LazyValue<T> {
    type Value<'tcx> = LazyValue<T::Value<'tcx>>;
}
/* AST_META: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4 */

impl<T: ParameterizedOverTcx> ParameterizedOverTcx for LazyArray<T> {
    type Value<'tcx> = LazyArray<T::Value<'tcx>>;
}
/* AST_META: AST_ID=14 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=11 | LINES=11 */

macro_rules! trivially_parameterized_over_tcx {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl ParameterizedOverTcx for $ty {
                #[allow(unused_lifetimes)]
                type Value<'tcx> = $ty;
            }
        )*
    }
}
/* AST_META: AST_ID=15 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=7 | LINES=71 */

trivially_parameterized_over_tcx! {
    bool,
    u64,
    usize,
    std::string::String,
    // tidy-alphabetical-start
    crate::rmeta::AttrFlags,
    crate::rmeta::CrateDep,
    crate::rmeta::CrateHeader,
    crate::rmeta::CrateRoot,
    crate::rmeta::IncoherentImpls,
    crate::rmeta::RawDefId,
    crate::rmeta::TraitImpls,
    crate::rmeta::VariantData,
    crate::rustc_abi::ReprOptions,
    crate::rustc_ast::DelimArgs,
    crate::rustc_hir::Attribute,
    crate::rustc_hir::ConstStability,
    crate::rustc_hir::Constness,
    crate::rustc_hir::CoroutineKind,
    crate::rustc_hir::DefaultBodyStability,
    crate::rustc_hir::Defaultness,
    crate::rustc_hir::LangItem,
    crate::rustc_hir::OpaqueTyOrigin<crate::rustc_hir::def_id::DefId>,
    crate::rustc_hir::PreciseCapturingArgKind<Symbol, Symbol>,
    crate::rustc_hir::Safety,
    crate::rustc_hir::Stability,
    crate::rustc_hir::attrs::Deprecation,
    crate::rustc_hir::attrs::StrippedCfgItem<crate::rustc_hir::def_id::DefIndex>,
    crate::rustc_hir::def::DefKind,
    crate::rustc_hir::def::DocLinkResMap,
    crate::rustc_hir::def_id::DefId,
    crate::rustc_hir::def_id::DefIndex,
    crate::rustc_hir::definitions::DefKey,
    crate::rustc_index::bit_set::DenseBitSet<u32>,
    crate::rustc_middle::metadata::ModChild,
    crate::rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs,
    crate::rustc_middle::middle::debugger_visualizer::DebuggerVisualizerFile,
    crate::rustc_middle::middle::exported_symbols::SymbolExportInfo,
    crate::rustc_middle::middle::lib_features::FeatureStability,
    crate::rustc_middle::middle::resolve_bound_vars::ObjectLifetimeDefault,
    crate::rustc_middle::mir::ConstQualifs,
    crate::rustc_middle::ty::AnonConstKind,
    crate::rustc_middle::ty::AssocContainer,
    crate::rustc_middle::ty::AsyncDestructor,
    crate::rustc_middle::ty::Asyncness,
    crate::rustc_middle::ty::DeducedParamAttrs,
    crate::rustc_middle::ty::Destructor,
    crate::rustc_middle::ty::Generics,
    crate::rustc_middle::ty::ImplTraitInTraitData,
    crate::rustc_middle::ty::IntrinsicDef,
    crate::rustc_middle::ty::TraitDef,
    crate::rustc_middle::ty::Variance,
    crate::rustc_middle::ty::Visibility<DefIndex>,
    crate::rustc_middle::ty::adjustment::CoerceUnsizedInfo,
    crate::rustc_middle::ty::fast_reject::SimplifiedType,
    crate::rustc_session::config::TargetModifier,
    crate::rustc_session::cstore::ForeignModule,
    crate::rustc_session::cstore::LinkagePreference,
    crate::rustc_session::cstore::NativeLib,
    crate::rustc_span::ExpnData,
    crate::rustc_span::ExpnHash,
    crate::rustc_span::ExpnId,
    crate::rustc_span::Ident,
    crate::rustc_span::SourceFile,
    crate::rustc_span::Span,
    crate::rustc_span::Symbol,
    crate::rustc_span::hygiene::SyntaxContextKey,
    // tidy-alphabetical-end
}
/* AST_META: AST_ID=16 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=11 | LINES=12 */

// HACK(compiler-errors): This macro rule can only take a fake path,
// not a real, due to parsing ambiguity reasons.
macro_rules! parameterized_over_tcx {
    ($($( $fake_path:ident )::+ ),+ $(,)?) => {
        $(
            impl ParameterizedOverTcx for $( $fake_path )::+ <'static> {
                type Value<'tcx> = $( $fake_path )::+ <'tcx>;
            }
        )*
    }
}
/* AST_META: AST_ID=17 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=18 */

parameterized_over_tcx! {
    // tidy-alphabetical-start
    crate::rmeta::DefPathHashMapRef,
    crate::rustc_middle::middle::exported_symbols::ExportedSymbol,
    crate::rustc_middle::mir::Body,
    crate::rustc_middle::mir::CoroutineLayout,
    crate::rustc_middle::mir::interpret::ConstAllocation,
    crate::rustc_middle::ty::Clause,
    crate::rustc_middle::ty::Const,
    crate::rustc_middle::ty::ConstConditions,
    crate::rustc_middle::ty::FnSig,
    crate::rustc_middle::ty::GenericPredicates,
    crate::rustc_middle::ty::ImplTraitHeader,
    crate::rustc_middle::ty::TraitRef,
    crate::rustc_middle::ty::Ty,
    // tidy-alphabetical-end
}