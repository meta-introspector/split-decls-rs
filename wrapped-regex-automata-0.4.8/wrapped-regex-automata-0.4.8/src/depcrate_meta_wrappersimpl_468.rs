// Generated macro for impl_468 (impl)
macro_rules! Depcrate_meta_wrappersimpl_468 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_468"}
// Dependencies: {}
impl PikeVMCache { pub (crate) fn none () -> PikeVMCache { PikeVMCache (None) } pub (crate) fn new (builder : & PikeVM) -> PikeVMCache { PikeVMCache (Some (builder . get () . 0 . create_cache ())) } pub (crate) fn reset (& mut self , builder : & PikeVM) { self . 0 . as_mut () . unwrap () . reset (& builder . get () . 0) ; } pub (crate) fn memory_usage (& self) -> usize { self . 0 . as_ref () . map_or (0 , | c | c . memory_usage ()) } }
};
}
