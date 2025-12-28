macro_rules! NakedFunctionTestingAttribute {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_naked_functions_testing_attribute , code = E0736)] pub (crate) struct NakedFunctionTestingAttribute { # [primary_span] # [label (builtin_macros_naked_attribute)] pub naked_span : Span , # [label] pub testing_span : Span , }
    };
}

NakedFunctionTestingAttribute!();