// Generated macro for impl_display (function)
macro_rules! Depcrate_item_enumimpl_display {
() => {
// Module: crate::item_enum
// Provides: {"impl_display"}
// Dependencies: {}
fn impl_display (ident : & syn :: Ident , variants : & Variants) -> TokenStream { let vs = variants . iter () . filter (| v | is_unit (v)) . map (| v | (config_value_of_variant (v) , & v . ident)) ; let match_patterns = fold_quote (vs , | (s , v) | { quote ! { # ident ::# v => write ! (f , "{}" , # s) , } }) ; quote ! { use std :: fmt ; impl fmt :: Display for # ident { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { # match_patterns _ => unimplemented ! () , } } } } }
};
}
