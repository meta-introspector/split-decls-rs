// Generated macro for impl_21 (impl)
macro_rules! Depcrate_murmur3impl_21 {
() => {
// Module: crate::murmur3
// Provides: {"impl_21"}
// Dependencies: {}
impl Murmur3Hasher { # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `self.index.usize() + buf.len() <= 4`."] unsafe fn push (& mut self , buf : & [u8]) { let start = self . index . usize () ; let len = buf . len () ; for i in 0 .. len { unsafe { * self . buf . bytes . assume_init_mut () . get_unchecked_mut (start + i) = * buf . get_unchecked (i) ; } } self . index = Index :: from (start + len) ; } }
};
}
