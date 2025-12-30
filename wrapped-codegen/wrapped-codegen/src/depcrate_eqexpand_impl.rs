// Generated macro for expand_impl (function)
macro_rules! Depcrate_eqexpand_impl {
() => {
// Module: crate::eq
// Provides: {"expand_impl"}
// Dependencies: {}
fn expand_impl (defs : & Definitions , node : & Node) -> TokenStream { if node . ident == "Member" || node . ident == "Index" || node . ident == "Lifetime" { return TokenStream :: new () ; } let ident = Ident :: new (& node . ident , Span :: call_site ()) ; let cfg_features = cfg :: features (& node . features , "extra-traits") ; let eq = quote ! { # cfg_features impl Eq for crate ::# ident { } } ; let manual_partial_eq = node . data == Data :: Private ; if manual_partial_eq { return eq ; } let body = expand_impl_body (defs , node) ; let other = match & node . data { Data :: Enum (variants) if variants . is_empty () => quote ! (_other) , Data :: Struct (fields) if fields . values () . all (always_eq) => quote ! (_other) , _ => quote ! (other) , } ; quote ! { # eq # cfg_features impl PartialEq for crate ::# ident { fn eq (& self , # other : & Self) -> bool { # body } } } }
};
}
