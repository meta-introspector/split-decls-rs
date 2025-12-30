// Generated macro for expand_impl (function)
macro_rules! Depcrate_hashexpand_impl {
() => {
// Module: crate::hash
// Provides: {"expand_impl"}
// Dependencies: {}
fn expand_impl (defs : & Definitions , node : & Node) -> TokenStream { let manual_hash = node . data == Data :: Private || node . ident == "Member" || node . ident == "Index" || node . ident == "Lifetime" ; if manual_hash { return TokenStream :: new () ; } let ident = Ident :: new (& node . ident , Span :: call_site ()) ; let cfg_features = cfg :: features (& node . features , "extra-traits") ; let body = expand_impl_body (defs , node) ; let hasher = match & node . data { Data :: Struct (_) if body . is_empty () => quote ! (_state) , Data :: Enum (variants) if variants . is_empty () => quote ! (_state) , _ => quote ! (state) , } ; quote ! { # cfg_features impl Hash for crate ::# ident { fn hash < H > (& self , # hasher : & mut H) where H : Hasher , { # body } } } }
};
}
