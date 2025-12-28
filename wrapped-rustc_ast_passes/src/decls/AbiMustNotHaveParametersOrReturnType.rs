macro_rules! AbiMustNotHaveParametersOrReturnType {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_abi_must_not_have_parameters_or_return_type)] # [note] pub (crate) struct AbiMustNotHaveParametersOrReturnType { # [primary_span] pub spans : Vec < Span > , pub abi : ExternAbi , # [suggestion (ast_passes_suggestion , applicability = "maybe-incorrect" , code = "{padding}fn {symbol}()" , style = "verbose")] pub suggestion_span : Span , pub symbol : Symbol , pub padding : & 'static str , }
    };
}

AbiMustNotHaveParametersOrReturnType!()