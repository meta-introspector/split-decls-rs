// Generated macro for impl_34 (impl)
macro_rules! Depcrate_bridge_bufferimpl_34 {
() => {
// Module: crate::bridge::buffer
// Provides: {"impl_34"}
// Dependencies: {}
impl From < Vec < u8 > > for Buffer { fn from (v : Vec < u8 >) -> Self { let mut v = ManuallyDrop :: new (v) ; let (data , len , capacity) = (v . as_mut_ptr () , v . len () , v . capacity ()) ; fn to_vec (b : Buffer) -> Vec < u8 > { unsafe { let b = ManuallyDrop :: new (b) ; Vec :: from_raw_parts (b . data , b . len , b . capacity) } } extern "C" fn reserve (b : Buffer , additional : usize) -> Buffer { let mut v = to_vec (b) ; v . reserve (additional) ; Buffer :: from (v) } extern "C" fn drop (b : Buffer) { mem :: drop (to_vec (b)) ; } Buffer { data , len , capacity , reserve , drop } } }
};
}
