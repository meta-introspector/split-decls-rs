// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'a , T : FloatCore > From < & 'a T > for & 'a OrderedFloat < T > { # [inline] fn from (t : & 'a T) -> & 'a OrderedFloat < T > { unsafe { & * (t as * const T as * const OrderedFloat < T >) } } }
};
}
