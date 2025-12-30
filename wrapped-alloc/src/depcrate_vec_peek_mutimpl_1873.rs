// Generated macro for impl_1873 (impl)
macro_rules! Depcrate_vec_peek_mutimpl_1873 {
() => {
// Module: crate::vec::peek_mut
// Provides: {"impl_1873"}
// Dependencies: {}
impl < 'a , T > PeekMut < 'a , T > { pub (crate) fn new (vec : & 'a mut Vec < T >) -> Option < Self > { if vec . is_empty () { None } else { Some (Self { vec }) } } # [doc = " Removes the peeked value from the vector and returns it."] # [unstable (feature = "vec_peek_mut" , issue = "122742")] pub fn pop (self) -> T { unsafe { self . vec . pop () . unwrap_unchecked () } } }
};
}
