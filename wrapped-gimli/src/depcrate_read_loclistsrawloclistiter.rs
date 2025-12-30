// Generated macro for RawLocListIter (struct)
macro_rules! Depcrate_read_loclistsRawLocListIter {
() => {
// Module: crate::read::loclists
// Provides: {"RawLocListIter"}
// Dependencies: {}
# [doc = " A raw iterator over a location list."] # [doc = ""] # [doc = " This iterator does not perform any processing of the location entries,"] # [doc = " such as handling base addresses."] # [derive (Debug)] pub struct RawLocListIter < R : Reader > { input : R , encoding : Encoding , format : LocListsFormat , }
};
}
