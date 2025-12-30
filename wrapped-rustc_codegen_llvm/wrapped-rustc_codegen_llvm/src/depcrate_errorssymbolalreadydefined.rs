// Generated macro for SymbolAlreadyDefined (struct)
macro_rules! Depcrate_errorsSymbolAlreadyDefined {
() => {
// Module: crate::errors
// Provides: {"SymbolAlreadyDefined"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (codegen_llvm_symbol_already_defined)] pub (crate) struct SymbolAlreadyDefined < 'a > { # [primary_span] pub span : Span , pub symbol_name : & 'a str , }
};
}
