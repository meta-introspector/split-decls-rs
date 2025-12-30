// Generated macro for impl_179 (impl)
macro_rules! Depcrate_string_recordimpl_179 {
() => {
// Module: crate::string_record
// Provides: {"impl_179"}
// Dependencies: {}
impl < T : AsRef < [u8] > > PartialEq < Vec < T > > for & StringRecord { fn eq (& self , other : & Vec < T >) -> bool { self . 0 . iter_eq (other) } }
};
}
