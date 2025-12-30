// Generated macro for impl_33 (impl)
macro_rules! Depcrate_buf_buf_mutimpl_33 {
() => {
// Module: crate::buf::buf_mut
// Provides: {"impl_33"}
// Dependencies: {}
unsafe impl BufMut for Vec < u8 > { # [inline] fn remaining_mut (& self) -> usize { isize :: MAX as usize - self . len () } # [inline] unsafe fn advance_mut (& mut self , cnt : usize) { let len = self . len () ; let remaining = self . capacity () - len ; if remaining < cnt { panic_advance (& TryGetError { requested : cnt , available : remaining , }) ; } self . set_len (len + cnt) ; } # [inline] fn chunk_mut (& mut self) -> & mut UninitSlice { if self . capacity () == self . len () { self . reserve (64) ; } let cap = self . capacity () ; let len = self . len () ; let ptr = self . as_mut_ptr () ; unsafe { UninitSlice :: from_raw_parts_mut (ptr . add (len) , cap - len) } } # [inline] fn put < T : super :: Buf > (& mut self , mut src : T) where Self : Sized , { self . reserve (src . remaining ()) ; while src . has_remaining () { let s = src . chunk () ; let l = s . len () ; self . extend_from_slice (s) ; src . advance (l) ; } } # [inline] fn put_slice (& mut self , src : & [u8]) { self . extend_from_slice (src) ; } # [inline] fn put_bytes (& mut self , val : u8 , cnt : usize) { let new_len = self . len () . saturating_add (cnt) ; self . resize (new_len , val) ; } }
};
}
