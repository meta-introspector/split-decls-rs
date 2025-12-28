macro_rules! deps {
    () => {
        UnusedGenericParameterHelp!();
    };
}

macro_rules! UnusedGenericParameter {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_analysis_unused_generic_parameter)] pub (crate) struct UnusedGenericParameter { # [primary_span] # [label] pub span : Span , pub param_name : Ident , pub param_def_kind : & 'static str , # [label (hir_analysis_usage_spans)] pub usage_spans : Vec < Span > , # [subdiagnostic] pub help : UnusedGenericParameterHelp , # [help (hir_analysis_const_param_help)] pub const_param_help : bool , }
    };
}

UnusedGenericParameter!()