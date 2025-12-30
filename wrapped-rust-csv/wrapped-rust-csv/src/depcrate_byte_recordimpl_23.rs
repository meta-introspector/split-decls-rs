// Generated macro for impl_23 (impl)
macro_rules! Depcrate_byte_recordimpl_23 {
() => {
// Module: crate::byte_record
// Provides: {"impl_23"}
// Dependencies: {}
impl ops :: Index < usize > for ByteRecord { type Output = [u8] ; # [inline] fn index (& self , i : usize) -> & [u8] { self . get (i) . unwrap () } }
};
}
