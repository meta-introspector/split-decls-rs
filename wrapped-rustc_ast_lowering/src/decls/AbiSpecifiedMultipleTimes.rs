macro_rules! AbiSpecifiedMultipleTimes {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_abi_specified_multiple_times)] pub (crate) struct AbiSpecifiedMultipleTimes { # [primary_span] pub abi_span : Span , pub prev_name : Symbol , # [label] pub prev_span : Span , # [note] pub equivalent : bool , }
    };
}

AbiSpecifiedMultipleTimes!();