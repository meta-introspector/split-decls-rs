// Generated macro for impl_333 (impl)
macro_rules! Depcrateimpl_333 {
() => {
// Module: crate
// Provides: {"impl_333"}
// Dependencies: {}
impl < const MIN_ALIGN : usize > Iterator for ChunkRawIter < '_ , MIN_ALIGN > { type Item = (* mut u8 , usize) ; fn next (& mut self) -> Option < (* mut u8 , usize) > { unsafe { let foot = self . footer . as_ref () ; if foot . is_empty () { return None ; } let (ptr , len) = foot . as_raw_parts () ; self . footer = foot . prev . get () ; Some ((ptr as * mut u8 , len)) } } }
};
}
