// Generated macro for impl_1264 (impl)
macro_rules! Depcrate_stackimpl_1264 {
() => {
// Module: crate::stack
// Provides: {"impl_1264"}
// Dependencies: {}
impl < T : Stackable > iter :: IntoIterator for Stack < T > { type IntoIter = IntoIter < T > ; type Item = T ; fn into_iter (self) -> IntoIter < T > { let it = IntoIter { stack : self . 0 , idxs : 0 .. self . len () as LenType , } ; mem :: forget (self) ; it } }
};
}
