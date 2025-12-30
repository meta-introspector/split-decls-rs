// Generated macro for impl_169 (impl)
macro_rules! Depcrate_unrawimpl_169 {
() => {
// Module: crate::unraw
// Provides: {"impl_169"}
// Dependencies: {}
impl MemberUnraw { pub fn span (& self) -> Span { match self { MemberUnraw :: Named (ident) => ident . 0 . span () , MemberUnraw :: Unnamed (index) => index . span , } } }
};
}
