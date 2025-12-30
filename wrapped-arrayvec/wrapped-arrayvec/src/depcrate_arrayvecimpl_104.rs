// Generated macro for impl_104 (impl)
macro_rules! Depcrate_arrayvecimpl_104 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_104"}
// Dependencies: {}
# [doc = " Extend the `ArrayVec` with an iterator."] # [doc = ""] # [doc = " ***Panics*** if extending the vector exceeds its capacity."] impl < T , const CAP : usize > Extend < T > for ArrayVec < T , CAP > { # [doc = " Extend the `ArrayVec` with an iterator."] # [doc = ""] # [doc = " ***Panics*** if extending the vector exceeds its capacity."] # [track_caller] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { unsafe { self . extend_from_iter :: < _ , true > (iter) } } }
};
}
