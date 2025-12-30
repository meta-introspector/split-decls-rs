// Generated macro for impl_245 (impl)
macro_rules! Depcrate_vecimpl_245 {
() => {
// Module: crate::vec
// Provides: {"impl_245"}
// Dependencies: {}
impl HalfBitsVecExt for Vec < u16 > { # [inline] fn reinterpret_into < H > (mut self) -> Vec < H > where H : crate :: private :: SealedHalf , { let length = self . len () ; let capacity = self . capacity () ; let pointer = self . as_mut_ptr () as * mut H ; mem :: forget (self) ; unsafe { Vec :: from_raw_parts (pointer , length , capacity) } } }
};
}
