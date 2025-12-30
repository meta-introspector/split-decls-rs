// Generated macro for impl_2313 (impl)
macro_rules! Depcrate_rangeimpl_2313 {
() => {
// Module: crate::range
// Provides: {"impl_2313"}
// Dependencies: {}
impl From < NSRange > for Range < usize > { # [inline] fn from (nsrange : NSRange) -> Self { Self { start : nsrange . location , end : nsrange . end () , } } }
};
}
