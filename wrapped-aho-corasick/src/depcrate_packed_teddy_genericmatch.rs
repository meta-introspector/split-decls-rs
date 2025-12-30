// Generated macro for Match (struct)
macro_rules! Depcrate_packed_teddy_genericMatch {
() => {
// Module: crate::packed::teddy::generic
// Provides: {"Match"}
// Dependencies: {}
# [doc = " A match type specialized to the Teddy implementations below."] # [doc = ""] # [doc = " Essentially, instead of representing a match at byte offsets, we use"] # [doc = " raw pointers. This is because the implementations below operate on raw"] # [doc = " pointers, and so this is a more natural return type based on how the"] # [doc = " implementation works."] # [doc = ""] # [doc = " Also, the `PatternID` used here is a `u16`."] # [derive (Clone , Copy , Debug)] pub (crate) struct Match { pid : PatternID , start : * const u8 , end : * const u8 , }
};
}
