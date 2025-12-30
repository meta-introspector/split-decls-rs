// Generated macro for rg (function)
macro_rules! Depcraterg {
() => {
// Module: crate
// Provides: {"rg"}
// Dependencies: {}
# [proc_macro] # [decl (fn , name = "rg" , vis = "pub" , hash = "c5559555")] pub fn rg (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let description_content = description . value () ; let span = description . span () ; quote ! { eprintln ! ("\n🔍 RG! Conceptual search: \"{}\"\n" , # description_content) ; } . into () }
};
}
