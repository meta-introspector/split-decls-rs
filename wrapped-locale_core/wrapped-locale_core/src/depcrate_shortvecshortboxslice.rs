// Generated macro for ShortBoxSlice (struct)
macro_rules! Depcrate_shortvecShortBoxSlice {
() => {
// Module: crate::shortvec
// Provides: {"ShortBoxSlice"}
// Dependencies: {}
# [doc = " A boxed slice that supports no-allocation, constant values if length 0 or 1."] # [doc = ""] # [doc = " Supports mutation but always reallocs when mutated."] # [derive (Debug , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] pub (crate) struct ShortBoxSlice < T > (ShortBoxSliceInner < T >) ;
};
}
