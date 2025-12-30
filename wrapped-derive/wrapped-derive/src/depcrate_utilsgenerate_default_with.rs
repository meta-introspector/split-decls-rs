// Generated macro for generate_default_with (function)
macro_rules! Depcrate_utilsgenerate_default_with {
() => {
// Module: crate::utils
// Provides: {"generate_default_with"}
// Dependencies: {}
fn generate_default_with (lit : & LitStr) -> GeneratorResult < TokenStream > { let str = lit . value () ; let tokens : TokenStream = str . parse () . map_err (| err | GeneratorError :: Syn (syn :: Error :: from (err))) ? ; Ok (quote ! { (# tokens) }) }
};
}
