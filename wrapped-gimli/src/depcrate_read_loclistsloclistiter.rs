// Generated macro for LocListIter (struct)
macro_rules! Depcrate_read_loclistsLocListIter {
() => {
// Module: crate::read::loclists
// Provides: {"LocListIter"}
// Dependencies: {}
# [doc = " An iterator over a location list."] # [doc = ""] # [doc = " This iterator internally handles processing of base address selection entries"] # [doc = " and list end entries.  Thus, it only returns location entries that are valid"] # [doc = " and already adjusted for the base address."] # [derive (Debug)] pub struct LocListIter < R : Reader > { raw : RawLocListIter < R > , base_address : u64 , debug_addr : DebugAddr < R > , debug_addr_base : DebugAddrBase < R :: Offset > , }
};
}
