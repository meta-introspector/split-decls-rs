// Generated macro for extract (function)
macro_rules! Depcrateextract {
() => {
// Module: crate
// Provides: {"extract"}
// Dependencies: {}
# [proc_macro] pub fn extract (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let fixme_id = input_str . value () ; quote ! { { use std :: fs ; let dir = format ! ("extracted/fixme-{}" , # fixme_id . len ()) ; fs :: create_dir_all (& dir) . ok () ; println ! ("cargo:warning=🔧 Extracted: {}" , dir) ; } } . into () }
};
}
