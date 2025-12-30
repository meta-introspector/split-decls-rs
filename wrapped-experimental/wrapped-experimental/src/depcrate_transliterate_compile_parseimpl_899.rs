// Generated macro for impl_899 (impl)
macro_rules! Depcrate_transliterate_compile_parseimpl_899 {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"impl_899"}
// Dependencies: {}
impl ElementKind { pub (crate) fn skipped_in (self , location : ElementLocation) -> bool { # [expect (clippy :: match_like_matches_macro)] match (location , self) { (ElementLocation :: Source , Self :: Cursor) => true , (ElementLocation :: Target , Self :: AnchorStart | Self :: AnchorEnd) => true , _ => false , } } pub (crate) fn debug_str (self) -> & 'static str { match self { ElementKind :: Literal => "literal" , ElementKind :: VariableReference => "variable reference" , ElementKind :: BackReference => "back reference" , ElementKind :: Quantifier => "quantifier" , ElementKind :: Segment => "segment" , ElementKind :: UnicodeSet => "unicodeset" , ElementKind :: FunctionCall => "function call" , ElementKind :: Cursor => "cursor" , ElementKind :: AnchorStart => "start anchor" , ElementKind :: AnchorEnd => "end anchor" , } } }
};
}
