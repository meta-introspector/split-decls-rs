// Generated macro for impl_deserialize (function)
macro_rules! Depcrate_item_enumimpl_deserialize {
() => {
// Module: crate::item_enum
// Provides: {"impl_deserialize"}
// Dependencies: {}
fn impl_deserialize (ident : & syn :: Ident , variants : & Variants) -> TokenStream { let supported_vs = variants . iter () . filter (| v | is_unit (v)) ; let if_patterns = fold_quote (supported_vs , | v | { let config_value = config_value_of_variant (v) ; let variant_ident = & v . ident ; quote ! { if # config_value . eq_ignore_ascii_case (s) { return Ok (# ident ::# variant_ident) ; } } }) ; let supported_vs = variants . iter () . filter (| v | is_unit (v)) ; let allowed = fold_quote (supported_vs . map (config_value_of_variant) , | s | quote ! (# s ,)) ; quote ! { impl <'de > serde :: de :: Deserialize <'de > for # ident { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : serde :: Deserializer <'de >, { use serde :: de :: { Error , Visitor } ; use std :: marker :: PhantomData ; use std :: fmt ; struct StringOnly < T > (PhantomData < T >) ; impl <'de , T > Visitor <'de > for StringOnly < T > where T : serde :: Deserializer <'de > { type Value = String ; fn expecting (& self , formatter : & mut fmt :: Formatter <'_ >) -> fmt :: Result { formatter . write_str ("string") } fn visit_str < E > (self , value : & str) -> Result < String , E > { Ok (String :: from (value)) } } let s = & d . deserialize_string (StringOnly ::< D > (PhantomData)) ?; # if_patterns static ALLOWED : &'static [& str] = & [# allowed] ; Err (D :: Error :: unknown_variant (& s , ALLOWED)) } } } }
};
}
