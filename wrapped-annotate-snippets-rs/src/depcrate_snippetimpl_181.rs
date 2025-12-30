// Generated macro for impl_181 (impl)
macro_rules! Depcrate_snippetimpl_181 {
() => {
// Module: crate::snippet
// Provides: {"impl_181"}
// Dependencies: {}
impl AnnotationKind { # [doc = " Annotate a byte span within [`Snippet`]"] pub fn span < 'a > (self , span : Range < usize >) -> Annotation < 'a > { Annotation { span , label : None , kind : self , highlight_source : false , } } pub (crate) fn is_primary (& self) -> bool { matches ! (self , AnnotationKind :: Primary) } }
};
}
