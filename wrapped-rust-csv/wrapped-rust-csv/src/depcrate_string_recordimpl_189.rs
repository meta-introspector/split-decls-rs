// Generated macro for impl_189 (impl)
macro_rules! Depcrate_string_recordimpl_189 {
() => {
// Module: crate::string_record
// Provides: {"impl_189"}
// Dependencies: {}
impl < T : AsRef < str > > Extend < T > for StringRecord { # [inline] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { for x in iter { self . push_field (x . as_ref ()) ; } } }
};
}
