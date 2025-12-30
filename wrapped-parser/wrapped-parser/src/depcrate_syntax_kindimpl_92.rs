// Generated macro for impl_92 (impl)
macro_rules! Depcrate_syntax_kindimpl_92 {
() => {
// Module: crate::syntax_kind
// Provides: {"impl_92"}
// Dependencies: {}
impl SyntaxKind { # [inline] pub fn is_trivia (self) -> bool { matches ! (self , SyntaxKind :: WHITESPACE | SyntaxKind :: COMMENT) } # [doc = " Returns true if this is an identifier or a keyword."] # [inline] pub fn is_any_identifier (self) -> bool { self == SyntaxKind :: IDENT || self . is_keyword (Edition :: LATEST) } }
};
}
