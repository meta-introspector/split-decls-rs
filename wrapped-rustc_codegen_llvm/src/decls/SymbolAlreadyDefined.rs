macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! SymbolAlreadyDefined {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_llvm_symbol_already_defined)] pub (crate) struct SymbolAlreadyDefined < 'a > { # [primary_span] pub span : Span , pub symbol_name : & 'a str , }
    };
}

SymbolAlreadyDefined!()