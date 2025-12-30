// Generated macro for expand_impl (function)
macro_rules! Depcrate_debugexpand_impl {
() => {
// Module: crate::debug
// Provides: {"expand_impl"}
// Dependencies: {}
fn expand_impl (defs : & Definitions , node : & Node , syntax_tree_variants : & Set < & str >) -> TokenStream { let manual_debug = node . data == Data :: Private || node . ident == "LitBool" ; if manual_debug { return TokenStream :: new () ; } let type_name = & node . ident ; let ident = Ident :: new (type_name , Span :: call_site ()) ; let is_syntax_tree_variant = syntax_tree_variants . contains (type_name . as_str ()) ; let cfg_features = cfg :: features (& node . features , "extra-traits") ; let body = expand_impl_body (defs , node , syntax_tree_variants) ; let formatter = match & node . data { Data :: Enum (variants) if variants . is_empty () => quote ! (_formatter) , _ => quote ! (formatter) , } ; if is_syntax_tree_variant { let inherent_cfg_features = cfg :: features (& node . features , DocCfg :: None) ; quote ! { # cfg_features impl Debug for crate ::# ident { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { self . debug (formatter , # type_name) } } # inherent_cfg_features impl crate ::# ident { fn debug (& self , # formatter : & mut fmt :: Formatter , name : & str) -> fmt :: Result { # body } } } } else { quote ! { # cfg_features impl Debug for crate ::# ident { fn fmt (& self , # formatter : & mut fmt :: Formatter) -> fmt :: Result { # body } } } } }
};
}
