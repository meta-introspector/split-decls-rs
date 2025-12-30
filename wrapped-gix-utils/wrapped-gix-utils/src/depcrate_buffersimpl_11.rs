// Generated macro for impl_11 (impl)
macro_rules! Depcrate_buffersimpl_11 {
() => {
// Module: crate::buffers
// Provides: {"impl_11"}
// Dependencies: {}
# [doc = " Lifecycle"] impl Buffers { # [doc = " Use this if there is an input buffer `src` which isn't owned by you, but which should be used as source when"] # [doc = " asking for [`src_and_dest()`](WithForeignSource::src_and_dest())."] pub fn use_foreign_src < 'a , 'src > (& 'a mut self , src : & 'src [u8]) -> WithForeignSource < 'src , 'a > { self . clear () ; WithForeignSource { ro_src : Some (src) , src : & mut self . src , dest : & mut self . dest , } } }
};
}
