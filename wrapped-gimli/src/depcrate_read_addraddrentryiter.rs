// Generated macro for AddrEntryIter (struct)
macro_rules! Depcrate_read_addrAddrEntryIter {
() => {
// Module: crate::read::addr
// Provides: {"AddrEntryIter"}
// Dependencies: {}
# [doc = " An iterator over the addresses from a `.debug_addr` section."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] # [derive (Debug , Clone)] pub struct AddrEntryIter < R : Reader > { input : R , encoding : Encoding , }
};
}
