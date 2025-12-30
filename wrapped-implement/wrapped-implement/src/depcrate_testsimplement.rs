// Generated macro for implement (function)
macro_rules! Depcrate_testsimplement {
() => {
// Module: crate::tests
// Provides: {"implement"}
// Dependencies: {}
fn implement (attributes : TokenStream , item_tokens : TokenStream) -> String { let out_tokens = crate :: implement_core (attributes , item_tokens) ; let tokens_string = out_tokens . to_string () ; let out_string = rustfmt (& tokens_string) ; println ! ("// output of #[implement] :") ; println ! () ; println ! ("{}" , out_string) ; out_string }
};
}
