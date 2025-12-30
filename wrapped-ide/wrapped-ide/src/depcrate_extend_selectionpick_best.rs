// Generated macro for pick_best (function)
macro_rules! Depcrate_extend_selectionpick_best {
() => {
// Module: crate::extend_selection
// Provides: {"pick_best"}
// Dependencies: {}
fn pick_best (l : SyntaxToken , r : SyntaxToken) -> SyntaxToken { return if priority (& r) > priority (& l) { r } else { l } ; fn priority (n : & SyntaxToken) -> usize { match n . kind () { WHITESPACE => 0 , IDENT | T ! [self] | T ! [super] | T ! [crate] | T ! [Self] | LIFETIME_IDENT => 2 , _ => 1 , } } }
};
}
