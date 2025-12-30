// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a , T : FloatCore > From < & 'a mut T > for & 'a mut OrderedFloat < T > { # [inline] fn from (t : & 'a mut T) -> & 'a mut OrderedFloat < T > { unsafe { & mut * (t as * mut T as * mut OrderedFloat < T >) } } }
};
}
