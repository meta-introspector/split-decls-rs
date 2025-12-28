macro_rules! deps {
    () => {
        EmptyLabelManySpans!();
    };
}

macro_rules! ArgsBeforeConstraint {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (ast_passes_generic_before_constraints)] pub (crate) struct ArgsBeforeConstraint { # [primary_span] pub arg_spans : Vec < Span > , # [label (ast_passes_constraints)] pub constraints : Span , # [label (ast_passes_args)] pub args : Span , # [suggestion (code = "{suggestion}" , applicability = "machine-applicable" , style = "verbose")] pub data : Span , pub suggestion : String , pub constraint_len : usize , pub args_len : usize , # [subdiagnostic] pub constraint_spans : EmptyLabelManySpans , # [subdiagnostic] pub arg_spans2 : EmptyLabelManySpans , }
    };
}

ArgsBeforeConstraint!();