// Generated macro for impl_330 (impl)
macro_rules! Depcrateimpl_330 {
() => {
// Module: crate
// Provides: {"impl_330"}
// Dependencies: {}
impl < 'a , const MIN_ALIGN : usize > Iterator for ChunkIter < 'a , MIN_ALIGN > { type Item = & 'a [mem :: MaybeUninit < u8 >] ; fn next (& mut self) -> Option < Self :: Item > { unsafe { let (ptr , len) = self . raw . next () ? ; let slice = slice :: from_raw_parts (ptr as * const mem :: MaybeUninit < u8 > , len) ; Some (slice) } } }
};
}
