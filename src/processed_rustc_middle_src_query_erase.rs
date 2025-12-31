use std::ffi::OsStr;
use std::intrinsics::transmute_unchecked;
use std::mem::MaybeUninit;

use crate::rustc_complete::ErrorGuaranteed;

use crate::mir::interpret::EvalToValTreeResult;
use crate::query::CyclePlaceholder;
use crate::traits::solve;
use crate::ty::adjustment::CoerceUnsizedInfo;
use crate::ty::{self, Ty, TyCtxt};
use crate::{mir, traits};

#[derive(Copy, Clone)]
pub struct Erased<T: Copy> {
    // We use `MaybeUninit` here so we can store any value
    // in `data` since we aren't actually storing a `T`.
    data: MaybeUninit<T>,
}

pub trait EraseType: Copy {
    type Result: Copy;
}

// Allow `type_alias_bounds` since compilation will fail without `EraseType`.
#[allow(type_alias_bounds)]
pub type Erase<T: EraseType> = Erased<impl Copy>;

#[inline(always)]
#[define_opaque(Erase)]
pub fn erase<T: EraseType>(src: T) -> Erase<T> {
    // Ensure the sizes match
    const {
        if size_of::<T>() != size_of::<T::Result>() {
            panic!("size of T must match erased type T::Result")
        }
    };

    Erased::<<T as EraseType>::Result> {
        // `transmute_unchecked` is needed here because it does not have `transmute`'s size check
        // (and thus allows to transmute between `T` and `MaybeUninit<T::Result>`) (we do the size
        // check ourselves in the `const` block above).
        //
        // `transmute_copy` is also commonly used for this (and it would work here since
        // `EraseType: Copy`), but `transmute_unchecked` better explains the intent.
        //
        // SAFETY: It is safe to transmute to MaybeUninit for types with the same sizes.
        data: unsafe { transmute_unchecked::<T, MaybeUninit<T::Result>>(src) },
    }
}

/// Restores an erased value.
#[inline(always)]
#[define_opaque(Erase)]
pub fn restore<T: EraseType>(value: Erase<T>) -> T {
    let value: Erased<<T as EraseType>::Result> = value;
    // See comment in `erase` for why we use `transmute_unchecked`.
    //
    // SAFETY: Due to the use of impl Trait in `Erase` the only way to safely create an instance
    // of `Erase` is to call `erase`, so we know that `value.data` is a valid instance of `T` of
    // the right size.
    unsafe { transmute_unchecked::<MaybeUninit<T::Result>, T>(value.data) }
}

impl<T> EraseType for &'_ T {
    type Result = [u8; size_of::<&'static ()>()];
}

impl<T> EraseType for &'_ [T] {
    type Result = [u8; size_of::<&'static [()]>()];
}

impl EraseType for &'_ OsStr {
    type Result = [u8; size_of::<&'static OsStr>()];
}

impl<T> EraseType for &'_ ty::List<T> {
    type Result = [u8; size_of::<&'static ty::List<()>>()];
}

impl<T> EraseType for &'_ ty::ListWithCachedTypeInfo<T> {
    type Result = [u8; size_of::<&'static ty::ListWithCachedTypeInfo<()>>()];
}

impl<I: rustc_index::Idx, T> EraseType for &'_ rustc_index::IndexSlice<I, T> {
    type Result = [u8; size_of::<&'static rustc_index::IndexSlice<u32, ()>>()];
}

impl<T> EraseType for Result<&'_ T, traits::query::NoSolution> {
    type Result = [u8; size_of::<Result<&'static (), traits::query::NoSolution>>()];
}

impl<T> EraseType for Result<&'_ [T], traits::query::NoSolution> {
    type Result = [u8; size_of::<Result<&'static [()], traits::query::NoSolution>>()];
}

impl<T> EraseType for Result<&'_ T, crate::rustc_errors::ErrorGuaranteed> {
    type Result = [u8; size_of::<Result<&'static (), crate::rustc_errors::ErrorGuaranteed>>()];
}

impl<T> EraseType for Result<&'_ [T], crate::rustc_errors::ErrorGuaranteed> {
    type Result = [u8; size_of::<Result<&'static [()], crate::rustc_errors::ErrorGuaranteed>>()];
}

impl<T> EraseType for Result<&'_ T, traits::CodegenObligationError> {
    type Result = [u8; size_of::<Result<&'static (), traits::CodegenObligationError>>()];
}

impl<T> EraseType for Result<&'_ T, &'_ ty::layout::FnAbiError<'_>> {
    type Result = [u8; size_of::<Result<&'static (), &'static ty::layout::FnAbiError<'static>>>()];
}

impl<T> EraseType for Result<(&'_ T, crate::thir::ExprId), crate::rustc_errors::ErrorGuaranteed> {
    type Result = [u8; size_of::<
        Result<(&'static (), crate::thir::ExprId), crate::rustc_errors::ErrorGuaranteed>,
    >()];
}

impl EraseType for Result<Option<ty::Instance<'_>>, crate::rustc_errors::ErrorGuaranteed> {
    type Result =
        [u8; size_of::<Result<Option<ty::Instance<'static>>, crate::rustc_errors::ErrorGuaranteed>>()];
}

impl EraseType for Result<CoerceUnsizedInfo, crate::rustc_errors::ErrorGuaranteed> {
    type Result = [u8; size_of::<Result<CoerceUnsizedInfo, crate::rustc_errors::ErrorGuaranteed>>()];
}

impl EraseType
    for Result<Option<ty::EarlyBinder<'_, ty::Const<'_>>>, crate::rustc_errors::ErrorGuaranteed>
{
    type Result = [u8; size_of::<
        Result<Option<ty::EarlyBinder<'static, ty::Const<'static>>>, crate::rustc_errors::ErrorGuaranteed>,
    >()];
}

impl EraseType for Result<ty::GenericArg<'_>, traits::query::NoSolution> {
    type Result = [u8; size_of::<Result<ty::GenericArg<'static>, traits::query::NoSolution>>()];
}

impl EraseType for Result<bool, &ty::layout::LayoutError<'_>> {
    type Result = [u8; size_of::<Result<bool, &'static ty::layout::LayoutError<'static>>>()];
}

impl EraseType for Result<rustc_abi::TyAndLayout<'_, Ty<'_>>, &ty::layout::LayoutError<'_>> {
    type Result = [u8; size_of::<
        Result<
            rustc_abi::TyAndLayout<'static, Ty<'static>>,
            &'static ty::layout::LayoutError<'static>,
        >,
    >()];
}

impl EraseType for Result<mir::ConstAlloc<'_>, mir::interpret::ErrorHandled> {
    type Result = [u8; size_of::<Result<mir::ConstAlloc<'static>, mir::interpret::ErrorHandled>>()];
}

impl EraseType for Result<mir::ConstValue, mir::interpret::ErrorHandled> {
    type Result = [u8; size_of::<Result<mir::ConstValue, mir::interpret::ErrorHandled>>()];
}

impl EraseType for EvalToValTreeResult<'_> {
    type Result = [u8; size_of::<EvalToValTreeResult<'static>>()];
}

impl EraseType for Result<&'_ ty::List<Ty<'_>>, ty::util::AlwaysRequiresDrop> {
    type Result =
        [u8; size_of::<Result<&'static ty::List<Ty<'static>>, ty::util::AlwaysRequiresDrop>>()];
}

impl EraseType for Result<ty::EarlyBinder<'_, Ty<'_>>, CyclePlaceholder> {
    type Result = [u8; size_of::<Result<ty::EarlyBinder<'static, Ty<'_>>, CyclePlaceholder>>()];
}

impl<T> EraseType for Option<&'_ T> {
    type Result = [u8; size_of::<Option<&'static ()>>()];
}

impl<T> EraseType for Option<&'_ [T]> {
    type Result = [u8; size_of::<Option<&'static [()]>>()];
}

impl EraseType for Option<&'_ OsStr> {
    type Result = [u8; size_of::<Option<&'static OsStr>>()];
}

impl EraseType for Option<mir::DestructuredConstant<'_>> {
    type Result = [u8; size_of::<Option<mir::DestructuredConstant<'static>>>()];
}

impl EraseType for Option<ty::ImplTraitHeader<'_>> {
    type Result = [u8; size_of::<Option<ty::ImplTraitHeader<'static>>>()];
}

impl EraseType for Option<ty::EarlyBinder<'_, Ty<'_>>> {
    type Result = [u8; size_of::<Option<ty::EarlyBinder<'static, Ty<'static>>>>()];
}

impl EraseType for crate::rustc_hir::MaybeOwner<'_> {
    type Result = [u8; size_of::<crate::rustc_hir::MaybeOwner<'static>>()];
}

impl<T: EraseType> EraseType for ty::EarlyBinder<'_, T> {
    type Result = T::Result;
}

impl EraseType for ty::Binder<'_, ty::FnSig<'_>> {
    type Result = [u8; size_of::<ty::Binder<'static, ty::FnSig<'static>>>()];
}

impl EraseType for ty::Binder<'_, ty::CoroutineWitnessTypes<TyCtxt<'_>>> {
    type Result =
        [u8; size_of::<ty::Binder<'static, ty::CoroutineWitnessTypes<TyCtxt<'static>>>>()];
}

impl EraseType for ty::Binder<'_, &'_ ty::List<Ty<'_>>> {
    type Result = [u8; size_of::<ty::Binder<'static, &'static ty::List<Ty<'static>>>>()];
}

impl<T0, T1> EraseType for (&'_ T0, &'_ T1) {
    type Result = [u8; size_of::<(&'static (), &'static ())>()];
}

impl<T0> EraseType for (solve::QueryResult<'_>, &'_ T0) {
    type Result = [u8; size_of::<(solve::QueryResult<'static>, &'static ())>()];
}

impl<T0, T1> EraseType for (&'_ T0, &'_ [T1]) {
    type Result = [u8; size_of::<(&'static (), &'static [()])>()];
}

impl<T0, T1> EraseType for (&'_ [T0], &'_ [T1]) {
    type Result = [u8; size_of::<(&'static [()], &'static [()])>()];
}

impl<T0> EraseType for (&'_ T0, Result<(), ErrorGuaranteed>) {
    type Result = [u8; size_of::<(&'static (), Result<(), ErrorGuaranteed>)>()];
}

macro_rules! trivial {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl EraseType for $ty {
                type Result = [u8; size_of::<$ty>()];
            }
        )*
    }
}

trivial! {
    (),
    bool,
    Option<(crate::rustc_span::def_id::DefId, crate::rustc_session::config::EntryFnType)>,
    Option<crate::rustc_ast::expand::allocator::AllocatorKind>,
    Option<crate::rustc_hir::ConstStability>,
    Option<crate::rustc_hir::DefaultBodyStability>,
    Option<crate::rustc_hir::Stability>,
    Option<crate::rustc_data_structures::svh::Svh>,
    Option<crate::rustc_hir::def::DefKind>,
    Option<crate::rustc_hir::CoroutineKind>,
    Option<crate::rustc_hir::HirId>,
    Option<crate::rustc_middle::middle::stability::DeprecationEntry>,
    Option<crate::rustc_middle::ty::AsyncDestructor>,
    Option<crate::rustc_middle::ty::Destructor>,
    Option<crate::rustc_middle::ty::ImplTraitInTraitData>,
    Option<crate::rustc_middle::ty::ScalarInt>,
    Option<crate::rustc_span::def_id::CrateNum>,
    Option<crate::rustc_span::def_id::DefId>,
    Option<crate::rustc_span::def_id::LocalDefId>,
    Option<crate::rustc_span::Span>,
    Option<rustc_abi::FieldIdx>,
    Option<rustc_target::spec::PanicStrategy>,
    Option<usize>,
    Option<crate::rustc_middle::ty::IntrinsicDef>,
    Option<rustc_abi::Align>,
    Result<(), crate::rustc_errors::ErrorGuaranteed>,
    Result<(), crate::rustc_middle::traits::query::NoSolution>,
    Result<crate::rustc_middle::traits::EvaluationResult, crate::rustc_middle::traits::OverflowError>,
    rustc_abi::ReprOptions,
    crate::rustc_ast::expand::allocator::AllocatorKind,
    crate::rustc_hir::DefaultBodyStability,
    crate::rustc_hir::attrs::Deprecation,
    crate::rustc_data_structures::svh::Svh,
    crate::rustc_errors::ErrorGuaranteed,
    crate::rustc_hir::Constness,
    crate::rustc_hir::ConstStability,
    crate::rustc_hir::def_id::DefId,
    crate::rustc_hir::def_id::DefIndex,
    crate::rustc_hir::def_id::LocalDefId,
    crate::rustc_hir::def_id::LocalModDefId,
    crate::rustc_hir::def::DefKind,
    crate::rustc_hir::Defaultness,
    crate::rustc_hir::definitions::DefKey,
    crate::rustc_hir::CoroutineKind,
    crate::rustc_hir::HirId,
    crate::rustc_hir::IsAsync,
    crate::rustc_hir::ItemLocalId,
    crate::rustc_hir::LangItem,
    crate::rustc_hir::OpaqueTyOrigin<crate::rustc_hir::def_id::DefId>,
    crate::rustc_hir::OwnerId,
    crate::rustc_hir::Stability,
    crate::rustc_hir::Upvar,
    rustc_index::bit_set::FiniteBitSet<u32>,
    crate::rustc_middle::middle::dependency_format::Linkage,
    crate::rustc_middle::middle::exported_symbols::SymbolExportInfo,
    crate::rustc_middle::middle::resolve_bound_vars::ObjectLifetimeDefault,
    crate::rustc_middle::middle::resolve_bound_vars::ResolvedArg,
    crate::rustc_middle::middle::stability::DeprecationEntry,
    crate::rustc_middle::mir::ConstQualifs,
    crate::rustc_middle::mir::ConstValue,
    crate::rustc_middle::mir::interpret::AllocId,
    crate::rustc_middle::mir::interpret::CtfeProvenance,
    crate::rustc_middle::mir::interpret::ErrorHandled,
    crate::rustc_middle::thir::ExprId,
    crate::rustc_middle::traits::CodegenObligationError,
    crate::rustc_middle::traits::EvaluationResult,
    crate::rustc_middle::traits::OverflowError,
    crate::rustc_middle::traits::query::NoSolution,
    crate::rustc_middle::traits::WellFormedLoc,
    crate::rustc_middle::ty::adjustment::CoerceUnsizedInfo,
    crate::rustc_middle::ty::AssocItem,
    crate::rustc_middle::ty::AssocContainer,
    crate::rustc_middle::ty::Asyncness,
    crate::rustc_middle::ty::AsyncDestructor,
    crate::rustc_middle::ty::BoundVariableKind,
    crate::rustc_middle::ty::AnonConstKind,
    crate::rustc_middle::ty::DeducedParamAttrs,
    crate::rustc_middle::ty::Destructor,
    crate::rustc_middle::ty::fast_reject::SimplifiedType,
    crate::rustc_middle::ty::ImplPolarity,
    crate::rustc_middle::ty::Representability,
    crate::rustc_middle::ty::UnusedGenericParams,
    crate::rustc_middle::ty::util::AlwaysRequiresDrop,
    crate::rustc_middle::ty::Visibility<crate::rustc_span::def_id::DefId>,
    crate::rustc_session::config::CrateType,
    crate::rustc_session::config::EntryFnType,
    crate::rustc_session::config::OptLevel,
    crate::rustc_session::config::SymbolManglingVersion,
    crate::rustc_session::cstore::CrateDepKind,
    crate::rustc_session::cstore::ExternCrate,
    crate::rustc_session::cstore::LinkagePreference,
    crate::rustc_session::Limits,
    crate::rustc_session::lint::LintExpectationId,
    crate::rustc_span::def_id::CrateNum,
    crate::rustc_span::def_id::DefPathHash,
    crate::rustc_span::ExpnHash,
    crate::rustc_span::ExpnId,
    crate::rustc_span::Span,
    crate::rustc_span::Symbol,
    crate::rustc_span::Ident,
    rustc_target::spec::PanicStrategy,
    rustc_target::spec::SanitizerSet,
    rustc_type_ir::Variance,
    u32,
    usize,
}

macro_rules! tcx_lifetime {
    ($($($fake_path:ident)::+),+ $(,)?) => {
        $(
            impl<'tcx> EraseType for $($fake_path)::+<'tcx> {
                type Result = [u8; size_of::<$($fake_path)::+<'static>>()];
            }
        )*
    }
}

tcx_lifetime! {
    crate::rustc_middle::middle::exported_symbols::ExportedSymbol,
    crate::rustc_middle::mir::Const,
    crate::rustc_middle::mir::DestructuredConstant,
    crate::rustc_middle::mir::ConstAlloc,
    crate::rustc_middle::mir::interpret::GlobalId,
    crate::rustc_middle::mir::interpret::LitToConstInput,
    crate::rustc_middle::mir::interpret::EvalStaticInitializerRawResult,
    crate::rustc_middle::mir::mono::MonoItemPartitions,
    crate::rustc_middle::traits::query::MethodAutoderefStepsResult,
    crate::rustc_middle::traits::query::type_op::AscribeUserType,
    crate::rustc_middle::traits::query::type_op::Eq,
    crate::rustc_middle::traits::query::type_op::ProvePredicate,
    crate::rustc_middle::traits::query::type_op::Subtype,
    crate::rustc_middle::ty::AdtDef,
    crate::rustc_middle::ty::AliasTy,
    crate::rustc_middle::ty::ClauseKind,
    crate::rustc_middle::ty::ClosureTypeInfo,
    crate::rustc_middle::ty::Const,
    crate::rustc_middle::ty::DestructuredConst,
    crate::rustc_middle::ty::ExistentialTraitRef,
    crate::rustc_middle::ty::FnSig,
    crate::rustc_middle::ty::GenericArg,
    crate::rustc_middle::ty::GenericPredicates,
    crate::rustc_middle::ty::ConstConditions,
    crate::rustc_middle::ty::inhabitedness::InhabitedPredicate,
    crate::rustc_middle::ty::Instance,
    crate::rustc_middle::ty::InstanceKind,
    crate::rustc_middle::ty::layout::FnAbiError,
    crate::rustc_middle::ty::layout::LayoutError,
    crate::rustc_middle::ty::ParamEnv,
    crate::rustc_middle::ty::TypingEnv,
    crate::rustc_middle::ty::Predicate,
    crate::rustc_middle::ty::SymbolName,
    crate::rustc_middle::ty::TraitRef,
    crate::rustc_middle::ty::Ty,
    crate::rustc_middle::ty::UnevaluatedConst,
    crate::rustc_middle::ty::ValTree,
    crate::rustc_middle::ty::VtblEntry,
}