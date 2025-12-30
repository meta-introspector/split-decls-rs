// Generated macro for impl_144 (impl)
macro_rules! Depcrate_shortvecimpl_144 {
() => {
// Module: crate::shortvec
// Provides: {"impl_144"}
// Dependencies: {}
impl < T > IntoIterator for ShortBoxSlice < T > { type Item = T ; type IntoIter = ShortBoxSliceIntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { match self . 0 { ShortBoxSliceInner :: ZeroOne (option) => { ShortBoxSliceIntoIter (ShortBoxSliceIntoIterInner :: ZeroOne (option)) } # [cfg (feature = "alloc")] ShortBoxSliceInner :: Multi (boxed_slice) => ShortBoxSliceIntoIter (ShortBoxSliceIntoIterInner :: Multi (boxed_slice . into_vec () . into_iter ()) ,) , # [cfg (not (feature = "alloc"))] ShortBoxSliceInner :: Two (arr) => { ShortBoxSliceIntoIter (ShortBoxSliceIntoIterInner :: Two (arr . into_iter ())) } } } }
};
}
