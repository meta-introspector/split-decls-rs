// Generated macro for impl_35 (impl)
macro_rules! Depcrate_arrayimpl_35 {
() => {
// Module: crate::array
// Provides: {"impl_35"}
// Dependencies: {}
impl < T : Type > IntoIterator for CFRetained < CFArray < T > > { type Item = CFRetained < T > ; type IntoIter = CFArrayIntoIter < T > ; # [inline] fn into_iter (self) -> Self :: IntoIter { CFArrayIntoIter { array : self , index : 0 , } } }
};
}
