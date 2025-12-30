// Generated macro for process_match (function)
macro_rules! Depcrateprocess_match {
() => {
// Module: crate
// Provides: {"process_match"}
// Dependencies: {}
# [proc_macro] # [decl (fn , name = "process_match" , vis = "pub" , hash = "c80b5286")] pub fn process_match (input : TokenStream) -> TokenStream { let ProcessMatchInput { file , line , column , text , .. } = parse_macro_input ! (input as ProcessMatchInput) ; let output = quote ! { # [allow (unused_macros)] macro_rules ! log_match_info { () => { } ; } log_match_info ! () ; } ; output . into () }
};
}
