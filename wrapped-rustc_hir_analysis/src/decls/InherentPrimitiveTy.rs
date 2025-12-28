macro_rules! deps {
    () => {
        InherentPrimitiveTyNote!();
    };
}

macro_rules! InherentPrimitiveTy {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_analysis_inherent_primitive_ty , code = E0390)] # [help] pub (crate) struct InherentPrimitiveTy < 'a > { # [primary_span] pub span : Span , # [subdiagnostic] pub note : Option < InherentPrimitiveTyNote < 'a > > , }
    };
}

InherentPrimitiveTy!();