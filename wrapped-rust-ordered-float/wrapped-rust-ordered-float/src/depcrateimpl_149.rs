// Generated macro for impl_149 (impl)
macro_rules! Depcrateimpl_149 {
() => {
// Module: crate
// Provides: {"impl_149"}
// Dependencies: {}
impl From < NotNan < f32 > > for NotNan < f64 > { # [inline] fn from (v : NotNan < f32 >) -> NotNan < f64 > { unsafe { NotNan :: new_unchecked (v . 0 as f64) } } }
};
}
