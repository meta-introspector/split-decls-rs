// Generated macro for impl_123 (impl)
macro_rules! Depcrate_ext_sliceimpl_123 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_123"}
// Dependencies: {}
impl < 'h , 's > Iterator for SplitNReverse < 'h , 's > { type Item = & 'h [u8] ; # [inline] fn next (& mut self) -> Option < & 'h [u8] > { self . count += 1 ; if self . count > self . limit || self . split . done { None } else if self . count == self . limit { Some (& self . split . finder . haystack () [.. self . split . last]) } else { self . split . next () } } }
};
}
