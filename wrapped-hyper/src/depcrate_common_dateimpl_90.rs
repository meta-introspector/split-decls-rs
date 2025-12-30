// Generated macro for impl_90 (impl)
macro_rules! Depcrate_common_dateimpl_90 {
() => {
// Module: crate::common::date
// Provides: {"impl_90"}
// Dependencies: {}
impl fmt :: Write for CachedDate { fn write_str (& mut self , s : & str) -> fmt :: Result { let len = s . len () ; self . bytes [self . pos .. self . pos + len] . copy_from_slice (s . as_bytes ()) ; self . pos += len ; Ok (()) } }
};
}
