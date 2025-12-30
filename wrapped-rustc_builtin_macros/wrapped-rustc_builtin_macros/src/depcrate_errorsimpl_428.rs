// Generated macro for impl_428 (impl)
macro_rules! Depcrate_errorsimpl_428 {
() => {
// Module: crate::errors
// Provides: {"impl_428"}
// Dependencies: {}
impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for AsmClobberNoReg { fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let lbl1 = dcx . eagerly_translate_to_string (crate :: fluent_generated :: builtin_macros_asm_clobber_abi , [] . into_iter () ,) ; let lbl2 = dcx . eagerly_translate_to_string (crate :: fluent_generated :: builtin_macros_asm_clobber_outputs , [] . into_iter () ,) ; Diag :: new (dcx , level , crate :: fluent_generated :: builtin_macros_asm_clobber_no_reg) . with_span (self . spans . clone ()) . with_span_labels (self . clobbers , & lbl1) . with_span_labels (self . spans , & lbl2) } }
};
}
