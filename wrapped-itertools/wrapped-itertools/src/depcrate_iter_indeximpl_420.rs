// Generated macro for impl_420 (impl)
macro_rules! Depcrate_iter_indeximpl_420 {
() => {
// Module: crate::iter_index
// Provides: {"impl_420"}
// Dependencies: {}
impl < I > IteratorIndex < I > for RangeInclusive < usize > where I : Iterator , { type Output = Take < Skip < I > > ; fn index (self , iter : I) -> Self :: Output { let length = if * self . end () == usize :: MAX { assert_ne ! (* self . start () , 0) ; self . end () - self . start () + 1 } else { (self . end () + 1) . saturating_sub (* self . start ()) } ; iter . skip (* self . start ()) . take (length) } }
};
}
