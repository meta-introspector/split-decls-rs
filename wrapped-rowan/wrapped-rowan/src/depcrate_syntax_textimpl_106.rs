// Generated macro for impl_106 (impl)
macro_rules! Depcrate_syntax_textimpl_106 {
() => {
// Module: crate::syntax_text
// Provides: {"impl_106"}
// Dependencies: {}
impl PartialEq for SyntaxText { fn eq (& self , other : & SyntaxText) -> bool { if self . range . len () != other . range . len () { return false ; } let mut lhs = self . tokens_with_ranges () ; let mut rhs = other . tokens_with_ranges () ; zip_texts (& mut lhs , & mut rhs) . is_none () && lhs . all (| it | it . 1 . is_empty ()) && rhs . all (| it | it . 1 . is_empty ()) } }
};
}
