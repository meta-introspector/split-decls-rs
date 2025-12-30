// Generated macro for impl_186 (impl)
macro_rules! Depcrate_string_recordimpl_186 {
() => {
// Module: crate::string_record
// Provides: {"impl_186"}
// Dependencies: {}
impl < T : AsRef < str > > From < Vec < T > > for StringRecord { # [inline] fn from (xs : Vec < T >) -> StringRecord { StringRecord :: from_iter (xs) } }
};
}
