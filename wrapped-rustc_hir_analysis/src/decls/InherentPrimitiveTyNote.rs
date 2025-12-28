macro_rules! InherentPrimitiveTyNote {
    () => {
        # [derive (Subdiagnostic)] # [note (hir_analysis_inherent_primitive_ty_note)] pub (crate) struct InherentPrimitiveTyNote < 'a > { pub subty : Ty < 'a > , }
    };
}

InherentPrimitiveTyNote!();