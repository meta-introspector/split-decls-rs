// Generated macro for expand_impl (function)
macro_rules! Depcrate_snapshotexpand_impl {
() => {
// Module: crate::snapshot
// Provides: {"expand_impl"}
// Dependencies: {}
fn expand_impl (defs : & Definitions , node : & Node) -> TokenStream { let ident = Ident :: new (& node . ident , Span :: call_site ()) ; let body = expand_impl_body (defs , node , & node . ident , & Owned (quote ! (self . value))) ; let formatter = match & node . data { Data :: Enum (variants) if variants . is_empty () => quote ! (_formatter) , _ => quote ! (formatter) , } ; quote ! { impl Debug for Lite < syn ::# ident > { fn fmt (& self , # formatter : & mut fmt :: Formatter) -> fmt :: Result { # body } } } }
};
}
