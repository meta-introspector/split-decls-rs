// Generated macro for derive (function)
macro_rules! Depcrate_expandderive {
() => {
// Module: crate::expand
// Provides: {"derive"}
// Dependencies: {}
pub fn derive (input : & DeriveInput) -> TokenStream { match try_expand (input) { Ok (expanded) => expanded , Err (error) => fallback :: expand (input , error) , } }
};
}
