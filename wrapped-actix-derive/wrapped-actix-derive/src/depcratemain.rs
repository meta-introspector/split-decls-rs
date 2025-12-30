// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [doc = " Marks async main function as the `actix` system entry-point."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[actix::main]"] # [doc = " async fn main() {"] # [doc = "     println!(\"Hello world\");"] # [doc = " }"] # [doc = " ```"] # [proc_macro_attribute] pub fn main (_ : TokenStream , item : TokenStream) -> TokenStream { let mut output : TokenStream = (quote ! { # [:: actix :: __private :: main (system = "::actix::System")] }) . into () ; output . extend (item) ; output }
};
}
