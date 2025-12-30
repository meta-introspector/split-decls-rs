// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
# [doc = " Adds a float directly."] impl < T : FloatCore + Sum > Sum for OrderedFloat < T > { fn sum < I : Iterator < Item = OrderedFloat < T > > > (iter : I) -> Self { OrderedFloat (iter . map (| v | v . 0) . sum ()) } }
};
}
