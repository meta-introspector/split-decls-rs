// Generated macro for impl_154 (impl)
macro_rules! Depcrateimpl_154 {
() => {
// Module: crate
// Provides: {"impl_154"}
// Dependencies: {}
# [doc = " Adds a float directly."] # [doc = ""] # [doc = " Panics if the provided value is NaN."] impl < T : FloatCore + Sum > Sum for NotNan < T > { fn sum < I : Iterator < Item = NotNan < T > > > (iter : I) -> Self { NotNan :: new (iter . map (| v | v . 0) . sum ()) . expect ("Sum resulted in NaN") } }
};
}
