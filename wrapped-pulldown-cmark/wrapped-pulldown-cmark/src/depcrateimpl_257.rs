// Generated macro for impl_257 (impl)
macro_rules! Depcrateimpl_257 {
() => {
// Module: crate
// Provides: {"impl_257"}
// Dependencies: {}
impl < 'a > CodeBlockKind < 'a > { pub fn is_indented (& self) -> bool { matches ! (* self , CodeBlockKind :: Indented) } pub fn is_fenced (& self) -> bool { matches ! (* self , CodeBlockKind :: Fenced (_)) } pub fn into_static (self) -> CodeBlockKind < 'static > { match self { CodeBlockKind :: Indented => CodeBlockKind :: Indented , CodeBlockKind :: Fenced (s) => CodeBlockKind :: Fenced (s . into_static ()) , } } }
};
}
