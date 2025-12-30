// Generated macro for impl_32 (impl)
macro_rules! Depcrate_buf_buf_mutimpl_32 {
() => {
// Module: crate::buf::buf_mut
// Provides: {"impl_32"}
// Dependencies: {}
unsafe impl BufMut for & mut [core :: mem :: MaybeUninit < u8 >] { # [inline] fn remaining_mut (& self) -> usize { self . len () } # [inline] fn chunk_mut (& mut self) -> & mut UninitSlice { UninitSlice :: uninit (self) } # [inline] unsafe fn advance_mut (& mut self , cnt : usize) { if self . len () < cnt { panic_advance (& TryGetError { requested : cnt , available : self . len () , }) ; } let (_ , b) = core :: mem :: take (self) . split_at_mut (cnt) ; * self = b ; } # [inline] fn put_slice (& mut self , src : & [u8]) { if self . len () < src . len () { panic_advance (& TryGetError { requested : src . len () , available : self . len () , }) ; } unsafe { ptr :: copy_nonoverlapping (src . as_ptr () , self . as_mut_ptr () . cast () , src . len ()) ; self . advance_mut (src . len ()) ; } } # [inline] fn put_bytes (& mut self , val : u8 , cnt : usize) { if self . len () < cnt { panic_advance (& TryGetError { requested : cnt , available : self . len () , }) ; } unsafe { ptr :: write_bytes (self . as_mut_ptr () as * mut u8 , val , cnt) ; self . advance_mut (cnt) ; } } }
};
}
