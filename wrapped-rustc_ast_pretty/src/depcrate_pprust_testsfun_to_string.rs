// Generated macro for fun_to_string (function)
macro_rules! Depcrate_pprust_testsfun_to_string {
() => {
// Module: crate::pprust::tests
// Provides: {"fun_to_string"}
// Dependencies: {}
fn fun_to_string (decl : & ast :: FnDecl , header : ast :: FnHeader , ident : Ident , generics : & ast :: Generics ,) -> String { to_string (| s | { let (cb , ib) = s . head ("") ; s . print_fn (decl , header , Some (ident) , generics) ; s . end (ib) ; s . end (cb) ; }) }
};
}
