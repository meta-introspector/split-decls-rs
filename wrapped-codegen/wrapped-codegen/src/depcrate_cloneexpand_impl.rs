// Generated macro for expand_impl (function)
macro_rules! Depcrate_cloneexpand_impl {
() => {
// Module: crate::clone
// Provides: {"expand_impl"}
// Dependencies: {}
fn expand_impl (defs : & Definitions , node : & Node) -> TokenStream { let manual_clone = node . data == Data :: Private || node . ident == "Lifetime" ; if manual_clone { return TokenStream :: new () ; } let ident = Ident :: new (& node . ident , Span :: call_site ()) ; let cfg_features = cfg :: features (& node . features , "clone-impls") ; let copy = node . ident == "AttrStyle" || node . ident == "BinOp" || node . ident == "RangeLimits" || node . ident == "TraitBoundModifier" || node . ident == "UnOp" ; if copy { return quote ! { # cfg_features impl Copy for crate ::# ident { } # cfg_features impl Clone for crate ::# ident { fn clone (& self) -> Self { * self } } } ; } let body = expand_impl_body (defs , node) ; quote ! { # cfg_features impl Clone for crate ::# ident { fn clone (& self) -> Self { # body } } } }
};
}
