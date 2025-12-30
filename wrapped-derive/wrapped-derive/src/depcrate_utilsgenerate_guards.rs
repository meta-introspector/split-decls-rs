// Generated macro for generate_guards (function)
macro_rules! Depcrate_utilsgenerate_guards {
() => {
// Module: crate::utils
// Provides: {"generate_guards"}
// Dependencies: {}
pub fn generate_guards (crate_name : & TokenStream , expr : & Expr , map_err : TokenStream ,) -> GeneratorResult < TokenStream > { let code = quote ! { { use # crate_name :: GuardExt ; # expr } } ; Ok (quote ! { # crate_name :: Guard :: check (&# code , & ctx) . await # map_err ?; }) }
};
}
