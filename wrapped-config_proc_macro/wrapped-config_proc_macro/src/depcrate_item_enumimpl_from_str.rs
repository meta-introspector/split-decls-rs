// Generated macro for impl_from_str (function)
macro_rules! Depcrate_item_enumimpl_from_str {
() => {
// Module: crate::item_enum
// Provides: {"impl_from_str"}
// Dependencies: {}
fn impl_from_str (ident : & syn :: Ident , variants : & Variants) -> TokenStream { let vs = variants . iter () . filter (| v | is_unit (v)) . map (| v | (config_value_of_variant (v) , & v . ident)) ; let if_patterns = fold_quote (vs , | (s , v) | { quote ! { if # s . eq_ignore_ascii_case (s) { return Ok (# ident ::# v) ; } } }) ; let mut err_msg = String :: from ("Bad variant, expected one of:") ; for v in variants . iter () . filter (| v | is_unit (v)) { err_msg . push_str (& format ! (" `{}`" , v . ident)) ; } quote ! { impl :: std :: str :: FromStr for # ident { type Err = &'static str ; fn from_str (s : & str) -> Result < Self , Self :: Err > { # if_patterns return Err (# err_msg) ; } } } }
};
}
