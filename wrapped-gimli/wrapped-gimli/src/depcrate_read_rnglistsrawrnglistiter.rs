// Generated macro for RawRngListIter (struct)
macro_rules! Depcrate_read_rnglistsRawRngListIter {
() => {
// Module: crate::read::rnglists
// Provides: {"RawRngListIter"}
// Dependencies: {}
# [doc = " A raw iterator over an address range list."] # [doc = ""] # [doc = " This iterator does not perform any processing of the range entries,"] # [doc = " such as handling base addresses."] # [derive (Debug)] pub struct RawRngListIter < R : Reader > { input : R , encoding : Encoding , format : RangeListsFormat , }
};
}
