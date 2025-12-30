// Generated macro for impl_158 (impl)
macro_rules! Depcrateimpl_158 {
() => {
// Module: crate
// Provides: {"impl_158"}
// Dependencies: {}
impl < T : FloatCore + Product > Product for NotNan < T > { fn product < I : Iterator < Item = NotNan < T > > > (iter : I) -> Self { NotNan :: new (iter . map (| v | v . 0) . product ()) . expect ("Product resulted in NaN") } }
};
}
