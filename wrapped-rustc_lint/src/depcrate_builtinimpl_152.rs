// Generated macro for impl_152 (impl)
macro_rules! Depcrate_builtinimpl_152 {
() => {
// Module: crate::builtin
// Provides: {"impl_152"}
// Dependencies: {}
impl InitError { fn spanned (self , span : Span) -> InitError { Self { span : Some (span) , .. self } } fn nested (self , nested : impl Into < Option < InitError > >) -> InitError { assert ! (self . nested . is_none ()) ; Self { nested : nested . into () . map (Box :: new) , .. self } } }
};
}
