// Generated macro for message_response_derive_rtype (function)
macro_rules! Depcratemessage_response_derive_rtype {
() => {
// Module: crate
// Provides: {"message_response_derive_rtype"}
// Dependencies: {}
# [proc_macro_derive (MessageResponse)] pub fn message_response_derive_rtype (input : TokenStream) -> TokenStream { let ast : DeriveInput = syn :: parse (input) . unwrap () ; message_response :: expand (& ast) . into () }
};
}
