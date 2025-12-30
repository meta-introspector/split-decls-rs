// Generated macro for impl_119 (impl)
macro_rules! Depcrate_writersimpl_119 {
() => {
// Module: crate::writers
// Provides: {"impl_119"}
// Dependencies: {}
impl < W > WriterExt for W { fn header < H : ? Sized > (self , header : & H) -> HeaderWriter < '_ , H , Self > { HeaderWriter { inner : self , header , started : false , } } }
};
}
