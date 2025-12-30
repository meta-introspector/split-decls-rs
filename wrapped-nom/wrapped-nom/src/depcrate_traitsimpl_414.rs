// Generated macro for impl_414 (impl)
macro_rules! Depcrate_traitsimpl_414 {
() => {
// Module: crate::traits
// Provides: {"impl_414"}
// Dependencies: {}
impl < 'a , 'b > Compare < & 'b [u8] > for & 'a str { # [inline (always)] fn compare (& self , t : & 'b [u8]) -> CompareResult { AsBytes :: as_bytes (self) . compare (t) } # [inline (always)] fn compare_no_case (& self , t : & 'b [u8]) -> CompareResult { AsBytes :: as_bytes (self) . compare_no_case (t) } }
};
}
