// Generated macro for impl_187 (impl)
macro_rules! Depcrate_string_recordimpl_187 {
() => {
// Module: crate::string_record
// Provides: {"impl_187"}
// Dependencies: {}
impl < 'a , T : AsRef < str > > From < & 'a [T] > for StringRecord { # [inline] fn from (xs : & 'a [T]) -> StringRecord { StringRecord :: from_iter (xs) } }
};
}
