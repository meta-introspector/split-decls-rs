// Generated macro for should_emit_generic_error (function)
macro_rules! Depcrate_hir_ty_lowering_cmseshould_emit_generic_error {
() => {
// Module: crate::hir_ty_lowering::cmse
// Provides: {"should_emit_generic_error"}
// Dependencies: {}
fn should_emit_generic_error < 'tcx > (abi : ExternAbi , layout_err : & 'tcx LayoutError < 'tcx >) -> bool { use LayoutError :: * ; match layout_err { TooGeneric (ty) => { match abi { ExternAbi :: CmseNonSecureCall => { ! ty . is_impl_trait () } ExternAbi :: CmseNonSecureEntry => true , _ => bug ! ("invalid ABI: {abi}") , } } Unknown (..) | SizeOverflow (..) | NormalizationFailure (..) | ReferencesError (..) | Cycle (..) => { false } } }
};
}
