// Generated macro for KeyValue (enum)
macro_rules! Depcrate_parsersKeyValue {
() => {
// Module: crate::parsers
// Provides: {"KeyValue"}
// Dependencies: {}
# [derive (Debug)] pub enum KeyValue { Simple (Ident , LitStr) , Block (Ident , proc_macro2 :: TokenStream) , List (Ident , proc_macro2 :: TokenStream) , InlineTable (Ident , proc_macro2 :: TokenStream) , }
};
}
