// Generated macro for impl_431 (impl)
macro_rules! Depcrate_traitsimpl_431 {
() => {
// Module: crate::traits
// Provides: {"impl_431"}
// Dependencies: {}
impl < 'a , const N : usize > Compare < [u8 ; N] > for & 'a [u8] { # [inline (always)] fn compare (& self , t : [u8 ; N]) -> CompareResult { self . compare (& t [..]) } # [inline (always)] fn compare_no_case (& self , t : [u8 ; N]) -> CompareResult { self . compare_no_case (& t [..]) } }
};
}
