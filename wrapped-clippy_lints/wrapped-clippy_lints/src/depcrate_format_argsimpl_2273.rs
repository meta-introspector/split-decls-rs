// Generated macro for impl_2273 (impl)
macro_rules! Depcrate_format_argsimpl_2273 {
() => {
// Module: crate::format_args
// Provides: {"impl_2273"}
// Dependencies: {}
impl < 'tcx > FormatArgs < 'tcx > { pub fn new (tcx : TyCtxt < 'tcx > , conf : & 'static Conf , format_args : FormatArgsStorage) -> Self { let ty_msrv_map = make_ty_msrv_map (tcx) ; Self { format_args , msrv : conf . msrv , ignore_mixed : conf . allow_mixed_uninlined_format_args , ty_msrv_map , has_derived_debug : FxHashMap :: default () , has_pointer_format : FxHashMap :: default () , } } }
};
}
