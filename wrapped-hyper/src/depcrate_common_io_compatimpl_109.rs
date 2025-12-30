// Generated macro for impl_109 (impl)
macro_rules! Depcrate_common_io_compatimpl_109 {
() => {
// Module: crate::common::io::compat
// Provides: {"impl_109"}
// Dependencies: {}
impl < T > Compat < T > { pub (crate) fn new (io : T) -> Self { Compat (io) } fn p (self : Pin < & mut Self >) -> Pin < & mut T > { unsafe { self . map_unchecked_mut (| me | & mut me . 0) } } }
};
}
