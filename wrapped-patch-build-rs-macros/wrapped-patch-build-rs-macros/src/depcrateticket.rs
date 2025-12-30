// Generated macro for ticket (function)
macro_rules! Depcrateticket {
() => {
// Module: crate
// Provides: {"ticket"}
// Dependencies: {}
# [proc_macro] # [decl2 (fn , name = "ticket" , vis = "pub" , hash = "70cd56d5")] pub fn ticket (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let issue = input_str . value () ; quote ! { println ! ("🎫 Ticket: {}" , # issue) ; } . into () }
};
}
