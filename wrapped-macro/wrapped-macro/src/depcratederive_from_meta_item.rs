// Generated macro for derive_from_meta_item (function)
macro_rules! Depcratederive_from_meta_item {
() => {
// Module: crate
// Provides: {"derive_from_meta_item"}
// Dependencies: {}
# [proc_macro_derive (FromMetaItem , attributes (darling))] pub fn derive_from_meta_item (_input : TokenStream) -> TokenStream { Error :: custom ("darling::FromMetaItem has been replaced by darling::FromMeta") . write_errors () . into () }
};
}
