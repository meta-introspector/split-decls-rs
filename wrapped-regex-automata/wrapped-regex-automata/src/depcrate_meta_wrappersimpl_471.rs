// Generated macro for impl_471 (impl)
macro_rules! Depcrate_meta_wrappersimpl_471 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_471"}
// Dependencies: {}
impl PikeVMCache { pub (crate) fn none () -> PikeVMCache { PikeVMCache (None) } pub (crate) fn reset (& mut self , builder : & PikeVM) { self . get (& builder . get () . 0) . reset (& builder . get () . 0) ; } pub (crate) fn memory_usage (& self) -> usize { self . 0 . as_ref () . map_or (0 , | c | c . memory_usage ()) } fn get (& mut self , vm : & pikevm :: PikeVM) -> & mut pikevm :: Cache { self . 0 . get_or_insert_with (| | vm . create_cache ()) } }
};
}
