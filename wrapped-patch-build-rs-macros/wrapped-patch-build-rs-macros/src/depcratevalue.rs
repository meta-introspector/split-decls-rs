// Generated macro for value (function)
macro_rules! Depcratevalue {
() => {
// Module: crate
// Provides: {"value"}
// Dependencies: {}
# [proc_macro] pub fn value (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let amount = input_str . value () ; quote ! { println ! ("💰 Bounty: {}" , # amount) ; } . into () }
};
}
