macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! RequiresRustAbi {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_requires_rust_abi , code = E0737)] pub (crate) struct RequiresRustAbi { # [primary_span] pub span : Span , }
    };
}

RequiresRustAbi!();