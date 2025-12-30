// Generated macro for impl_112 (impl)
macro_rules! Depcrate_arrayvecimpl_112 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_112"}
// Dependencies: {}
impl < T , const CAP : usize > PartialEq < [T] > for ArrayVec < T , CAP > where T : PartialEq , { fn eq (& self , other : & [T]) -> bool { * * self == * other } }
};
}
