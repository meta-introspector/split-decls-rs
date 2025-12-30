// Generated macro for impl_36 (impl)
macro_rules! Depcrate_arrayimpl_36 {
() => {
// Module: crate::array
// Provides: {"impl_36"}
// Dependencies: {}
impl < T : Type > IntoIterator for CFRetained < CFMutableArray < T > > { type Item = CFRetained < T > ; type IntoIter = CFArrayIntoIter < T > ; # [inline] fn into_iter (self) -> Self :: IntoIter { let array = unsafe { CFRetained :: cast_unchecked :: < CFArray < T > > (self) } ; CFArrayIntoIter { array , index : 0 } } }
};
}
