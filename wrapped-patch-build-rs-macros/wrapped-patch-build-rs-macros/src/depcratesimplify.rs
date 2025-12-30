// Generated macro for simplify (function)
macro_rules! Depcratesimplify {
() => {
// Module: crate
// Provides: {"simplify"}
// Dependencies: {}
# [proc_macro] # [decl2 (fn , name = "simplify" , vis = "pub" , hash = "4e29d5fd")] pub fn simplify (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let data = input_str . value () ; quote ! { { let simplified = # data . lines () . filter (| line | ! line . trim () . is_empty ()) . filter (| line | ! line . starts_with ("//")) . collect ::< Vec < _ >> () . join ("\n") ; println ! ("cargo:warning=📉 Simplified: {} -> {} lines" , # data . lines () . count () , simplified . lines () . count ()) ; simplified } } . into () }
};
}
