// Generated macro for InPlaceSeed (struct)
macro_rules! Depcrate_vecInPlaceSeed {
() => {
// Module: crate::vec
// Provides: {"InPlaceSeed"}
// Dependencies: {}
# [doc = " A DeserializeSeed helper for implementing deserialize_in_place Visitors."] # [doc = ""] # [doc = " Wraps a mutable reference and calls deserialize_in_place on it."] # [cfg (feature = "serde")] pub struct InPlaceSeed < 'a , T : 'a > (pub & 'a mut T) ;
};
}
