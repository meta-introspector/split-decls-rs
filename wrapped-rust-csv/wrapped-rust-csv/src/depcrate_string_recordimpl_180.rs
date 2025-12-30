// Generated macro for impl_180 (impl)
macro_rules! Depcrate_string_recordimpl_180 {
() => {
// Module: crate::string_record
// Provides: {"impl_180"}
// Dependencies: {}
impl < T : AsRef < [u8] > > PartialEq < [T] > for StringRecord { fn eq (& self , other : & [T]) -> bool { self . 0 . iter_eq (other) } }
};
}
