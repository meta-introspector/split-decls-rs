// Generated macro for impl_521 (impl)
macro_rules! Depcrate_bufferimpl_521 {
() => {
// Module: crate::buffer
// Provides: {"impl_521"}
// Dependencies: {}
impl < 'a , T > Buffer < 'a , T > { pub (crate) fn new (owned : Vec < u8 >) -> Buffer < 'a , T > { Buffer (Cow :: Owned (owned) , PhantomData) } pub (crate) fn take_from_slice (slice : & mut [u8]) -> Buffer < 'a , T > { let owned = slice . to_vec () ; slice . zeroize () ; Buffer (Cow :: Owned (owned) , PhantomData) } }
};
}
