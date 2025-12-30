// Generated macro for impl_412 (impl)
macro_rules! Depcrate_traitsimpl_412 {
() => {
// Module: crate::traits
// Provides: {"impl_412"}
// Dependencies: {}
impl < 'a , 'b > Compare < & 'b str > for & 'a [u8] { # [inline (always)] fn compare (& self , t : & 'b str) -> CompareResult { self . compare (AsBytes :: as_bytes (t)) } # [inline (always)] fn compare_no_case (& self , t : & 'b str) -> CompareResult { self . compare_no_case (AsBytes :: as_bytes (t)) } }
};
}
