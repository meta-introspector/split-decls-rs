// Generated macro for ShortBoxSliceInner (enum)
macro_rules! Depcrate_shortvecShortBoxSliceInner {
() => {
// Module: crate::shortvec
// Provides: {"ShortBoxSliceInner"}
// Dependencies: {}
# [doc = " A boxed slice that supports no-allocation, constant values if length 0 or 1."] # [doc = " Using ZeroOne(Option<T>) saves 8 bytes in ShortBoxSlice via niche optimization."] # [derive (Debug , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] pub (crate) enum ShortBoxSliceInner < T > { ZeroOne (Option < T >) , # [cfg (feature = "alloc")] Multi (Box < [T] >) , # [cfg (not (feature = "alloc"))] Two ([T ; 2]) , }
};
}
