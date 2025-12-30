// Generated macro for AddrHeaderIter (struct)
macro_rules! Depcrate_read_addrAddrHeaderIter {
() => {
// Module: crate::read::addr
// Provides: {"AddrHeaderIter"}
// Dependencies: {}
# [doc = " An iterator over the headers of a `.debug_addr` section."] # [derive (Clone , Debug)] pub struct AddrHeaderIter < R : Reader > { input : R , offset : DebugAddrOffset < R :: Offset > , }
};
}
