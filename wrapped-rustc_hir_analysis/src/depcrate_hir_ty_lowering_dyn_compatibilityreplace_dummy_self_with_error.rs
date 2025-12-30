// Generated macro for replace_dummy_self_with_error (function)
macro_rules! Depcrate_hir_ty_lowering_dyn_compatibilityreplace_dummy_self_with_error {
() => {
// Module: crate::hir_ty_lowering::dyn_compatibility
// Provides: {"replace_dummy_self_with_error"}
// Dependencies: {}
fn replace_dummy_self_with_error < 'tcx , T : TypeFoldable < TyCtxt < 'tcx > > > (tcx : TyCtxt < 'tcx > , t : T , guar : ErrorGuaranteed ,) -> T { t . fold_with (& mut BottomUpFolder { tcx , ty_op : | ty | { if ty == tcx . types . trait_object_dummy_self { Ty :: new_error (tcx , guar) } else { ty } } , lt_op : | lt | lt , ct_op : | ct | ct , }) }
};
}
