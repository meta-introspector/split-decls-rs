// Generated macro for GenericArgsLowerer (trait)
macro_rules! Depcrate_hir_ty_loweringGenericArgsLowerer {
() => {
// Module: crate::hir_ty_lowering
// Provides: {"GenericArgsLowerer"}
// Dependencies: {}
# [doc = " A context which can lower HIR's [`GenericArg`] to `rustc_middle`'s [`ty::GenericArg`]."] # [doc = ""] # [doc = " Its only consumer is [`generics::lower_generic_args`]."] # [doc = " Read its documentation to learn more."] pub trait GenericArgsLowerer < 'a , 'tcx > { fn args_for_def_id (& mut self , def_id : DefId) -> (Option < & 'a GenericArgs < 'tcx > > , bool) ; fn provided_kind (& mut self , preceding_args : & [ty :: GenericArg < 'tcx >] , param : & ty :: GenericParamDef , arg : & GenericArg < 'tcx > ,) -> ty :: GenericArg < 'tcx > ; fn inferred_kind (& mut self , preceding_args : & [ty :: GenericArg < 'tcx >] , param : & ty :: GenericParamDef , infer_args : bool ,) -> ty :: GenericArg < 'tcx > ; }
};
}
