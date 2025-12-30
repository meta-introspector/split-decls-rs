// Generated macro for impl_246 (impl)
macro_rules! Depcrate_vecimpl_246 {
() => {
// Module: crate::vec
// Provides: {"impl_246"}
// Dependencies: {}
impl < T , A : Allocator > ops :: Deref for Vec < T , A > { type Target = [T] ; # [inline (always)] fn deref (& self) -> & [T] { unsafe { slice :: from_raw_parts (self . as_ptr () , self . len) } } }
};
}
