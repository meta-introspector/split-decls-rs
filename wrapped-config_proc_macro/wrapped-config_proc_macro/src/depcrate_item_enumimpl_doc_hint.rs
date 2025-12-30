// Generated macro for impl_doc_hint (function)
macro_rules! Depcrate_item_enumimpl_doc_hint {
() => {
// Module: crate::item_enum
// Provides: {"impl_doc_hint"}
// Dependencies: {}
fn impl_doc_hint (ident : & syn :: Ident , variants : & Variants) -> TokenStream { let doc_hint = variants . iter () . map (doc_hint_of_variant) . collect :: < Vec < _ > > () . join ("|") ; let doc_hint = format ! ("[{}]" , doc_hint) ; let variant_stables = variants . iter () . map (| v | (& v . ident , fields_in_variant (& v) , ! unstable_of_variant (v))) ; let match_patterns = fold_quote (variant_stables , | (v , fields , stable) | { quote ! { # ident ::# v # fields => # stable , } }) ; quote ! { use crate :: config :: ConfigType ; impl ConfigType for # ident { fn doc_hint () -> String { # doc_hint . to_owned () } fn stable_variant (& self) -> bool { match self { # match_patterns } } } } }
};
}
