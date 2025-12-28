macro_rules! ConstAndCVariadic {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_const_and_c_variadic)] pub (crate) struct ConstAndCVariadic { # [primary_span] pub spans : Vec < Span > , # [label (ast_passes_const)] pub const_span : Span , # [label (ast_passes_variadic)] pub variadic_span : Span , }
    };
}

ConstAndCVariadic!();