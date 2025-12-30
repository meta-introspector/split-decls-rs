// Generated macro for impl_index (macro)
macro_rules! Depcrate_buf_uninit_sliceimpl_index {
() => {
// Module: crate::buf::uninit_slice
// Provides: {"impl_index"}
// Dependencies: {}
macro_rules ! impl_index { ($ ($ t : ty) ,*) => { $ (impl Index <$ t > for UninitSlice { type Output = UninitSlice ; # [inline] fn index (& self , index : $ t) -> & UninitSlice { UninitSlice :: uninit_ref (& self . 0 [index]) } } impl IndexMut <$ t > for UninitSlice { # [inline] fn index_mut (& mut self , index : $ t) -> & mut UninitSlice { UninitSlice :: uninit (& mut self . 0 [index]) } }) * } ; }
};
}
