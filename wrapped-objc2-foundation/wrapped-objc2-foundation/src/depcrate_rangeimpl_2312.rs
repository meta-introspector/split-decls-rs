// Generated macro for impl_2312 (impl)
macro_rules! Depcrate_rangeimpl_2312 {
() => {
// Module: crate::range
// Provides: {"impl_2312"}
// Dependencies: {}
impl From < Range < usize > > for NSRange { fn from (range : Range < usize >) -> Self { let length = range . end . checked_sub (range . start) . expect ("Range end < start") ; Self { location : range . start , length , } } }
};
}
