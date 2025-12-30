// Generated macro for impl_80 (impl)
macro_rules! Depcrate_ppcimpl_80 {
() => {
// Module: crate::ppc
// Provides: {"impl_80"}
// Dependencies: {}
impl < F : FloatConvert < Self > > From < DoubleFloat < F > > for Fallback < F > { fn from (DoubleFloat (a , b) : DoubleFloat < F >) -> Self { let mut status ; let mut loses_info = false ; let a = unpack ! (status =, a . convert (& mut loses_info)) ; assert_eq ! ((status , loses_info) , (Status :: OK , false)) ; if a . is_finite_non_zero () { let b = unpack ! (status =, b . convert (& mut loses_info)) ; assert_eq ! ((status , loses_info) , (Status :: OK , false)) ; (a + b) . value } else { a } } }
};
}
