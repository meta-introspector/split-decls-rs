// Generated macro for impl_911 (impl)
macro_rules! Depcrate_transliterate_compile_parseimpl_911 {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"impl_911"}
// Dependencies: {}
impl Element { pub (crate) fn kind (& self) -> ElementKind { match self { Element :: Literal (..) => ElementKind :: Literal , Element :: VariableRef (..) => ElementKind :: VariableReference , Element :: BackRef (..) => ElementKind :: BackReference , Element :: Quantifier (..) => ElementKind :: Quantifier , Element :: Segment (..) => ElementKind :: Segment , Element :: UnicodeSet (..) => ElementKind :: UnicodeSet , Element :: FunctionCall (..) => ElementKind :: FunctionCall , Element :: Cursor (..) => ElementKind :: Cursor , Element :: AnchorStart => ElementKind :: AnchorStart , Element :: AnchorEnd => ElementKind :: AnchorEnd , } } }
};
}
