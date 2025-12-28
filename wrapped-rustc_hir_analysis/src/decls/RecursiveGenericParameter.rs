macro_rules! deps {
    () => {
        UnusedGenericParameterHelp!();
    };
}

macro_rules! RecursiveGenericParameter {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_analysis_recursive_generic_parameter)] pub (crate) struct RecursiveGenericParameter { # [primary_span] pub spans : Vec < Span > , # [label] pub param_span : Span , pub param_name : Ident , pub param_def_kind : & 'static str , # [subdiagnostic] pub help : UnusedGenericParameterHelp , # [note] pub note : () , }
    };
}

RecursiveGenericParameter!();