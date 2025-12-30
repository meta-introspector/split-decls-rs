// Generated macro for deserialize_match (macro)
macro_rules! Depcrate_serde_helpersdeserialize_match {
() => {
// Module: crate::serde_helpers
// Provides: {"deserialize_match"}
// Dependencies: {}
# [doc = " Helper macro that generates different match expressions depending on the presence"] # [doc = " of default variant"] # [macro_export] # [doc (hidden)] macro_rules ! deserialize_match { ($ tag : ident , $ de : ident , $ enum : ty , (_ => $ ($ default_variant : tt) +) $ (,) ?) => (Ok ($ crate :: deserialize_variant ! ($ de , $ enum , $ ($ default_variant) +))) ; ($ tag : ident , $ de : ident , $ enum : ty , $ (($ variant_tag : literal => $ ($ variant : tt) +)) ,* , (_ => $ ($ default_variant : tt) +) $ (,) ?) => (match $ tag . as_ref () { $ ($ variant_tag => Ok ($ crate :: deserialize_variant ! ($ de , $ enum , $ ($ variant) +)) ,) * _ => Ok ($ crate :: deserialize_variant ! ($ de , $ enum , $ ($ default_variant) +)) , }) ; ($ tag : ident , $ de : ident , $ enum : ty , $ (($ variant_tag : literal => $ ($ variant : tt) +)) ,* $ (,) ?) => (match $ tag . as_ref () { $ ($ variant_tag => Ok ($ crate :: deserialize_variant ! ($ de , $ enum , $ ($ variant) +)) ,) * _ => Err (A :: Error :: unknown_field (&$ tag , & [$ ($ variant_tag) ,+])) , }) ; }
};
}
