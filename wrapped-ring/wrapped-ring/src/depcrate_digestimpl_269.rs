// Generated macro for impl_269 (impl)
macro_rules! Depcrate_digestimpl_269 {
() => {
// Module: crate::digest
// Provides: {"impl_269"}
// Dependencies: {}
impl BlockLen { const MAX : Self = Self :: _1024 ; # [inline (always)] const fn into (self) -> usize { self as usize } # [inline (always)] const fn len_len (self) -> usize { let len_len = match self { BlockLen :: _512 => LenLen :: _64 , BlockLen :: _1024 => LenLen :: _128 , } ; len_len as usize } }
};
}
