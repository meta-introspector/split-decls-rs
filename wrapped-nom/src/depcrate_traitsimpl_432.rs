// Generated macro for impl_432 (impl)
macro_rules! Depcrate_traitsimpl_432 {
() => {
// Module: crate::traits
// Provides: {"impl_432"}
// Dependencies: {}
impl < 'a , 'b , const N : usize > Compare < & 'b [u8 ; N] > for & 'a [u8] { # [inline (always)] fn compare (& self , t : & 'b [u8 ; N]) -> CompareResult { self . compare (& t [..]) } # [inline (always)] fn compare_no_case (& self , t : & 'b [u8 ; N]) -> CompareResult { self . compare_no_case (& t [..]) } }
};
}
