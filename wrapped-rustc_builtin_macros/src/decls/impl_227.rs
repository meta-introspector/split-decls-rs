macro_rules! deps {
    () => {
        AsmClobberNoReg!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for AsmClobberNoReg { fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let lbl1 = dcx . eagerly_translate_to_string (crate :: fluent_generated :: builtin_macros_asm_clobber_abi , [] . into_iter () ,) ; let lbl2 = dcx . eagerly_translate_to_string (crate :: fluent_generated :: builtin_macros_asm_clobber_outputs , [] . into_iter () ,) ; Diag :: new (dcx , level , crate :: fluent_generated :: builtin_macros_asm_clobber_no_reg) . with_span (self . spans . clone ()) . with_span_labels (self . clobbers , & lbl1) . with_span_labels (self . spans , & lbl2) } }
    };
}

impl_227!();