// Generated macro for impl_120 (impl)
macro_rules! Depcrate_ext_sliceimpl_120 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'h , 's > Iterator for SplitN < 'h , 's > { type Item = & 'h [u8] ; # [inline] fn next (& mut self) -> Option < & 'h [u8] > { self . count += 1 ; if self . count > self . limit || self . split . done { None } else if self . count == self . limit { Some (& self . split . finder . haystack [self . split . last ..]) } else { self . split . next () } } }
};
}
