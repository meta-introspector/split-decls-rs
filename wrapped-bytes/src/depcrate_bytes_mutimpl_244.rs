// Generated macro for impl_244 (impl)
macro_rules! Depcrate_bytes_mutimpl_244 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_244"}
// Dependencies: {}
impl Drop for BytesMut { fn drop (& mut self) { let kind = self . kind () ; if kind == KIND_VEC { unsafe { let off = self . get_vec_pos () ; let _ = rebuild_vec (self . ptr . as_ptr () , self . len , self . cap , off) ; } } else if kind == KIND_ARC { unsafe { release_shared (self . data) } ; } } }
};
}
