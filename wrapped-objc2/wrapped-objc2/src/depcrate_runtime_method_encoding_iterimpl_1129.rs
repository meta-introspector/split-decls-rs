// Generated macro for impl_1129 (impl)
macro_rules! Depcrate_runtime_method_encoding_iterimpl_1129 {
() => {
// Module: crate::runtime::method_encoding_iter
// Provides: {"impl_1129"}
// Dependencies: {}
impl Iterator for MethodEncodingIter < '_ > { type Item = Result < (EncodingBox , Option < isize >) , EncodingParseError > ; fn next (& mut self) -> Option < Self :: Item > { if self . s . is_empty () { return None ; } Some (self . extract_encoding ()) } }
};
}
