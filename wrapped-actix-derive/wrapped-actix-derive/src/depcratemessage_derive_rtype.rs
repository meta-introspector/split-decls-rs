// Generated macro for message_derive_rtype (function)
macro_rules! Depcratemessage_derive_rtype {
() => {
// Module: crate
// Provides: {"message_derive_rtype"}
// Dependencies: {}
# [proc_macro_derive (Message , attributes (rtype))] pub fn message_derive_rtype (input : TokenStream) -> TokenStream { let ast : DeriveInput = syn :: parse (input) . unwrap () ; message :: expand (& ast) . into () }
};
}
