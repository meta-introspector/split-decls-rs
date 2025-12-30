// Generated macro for impl_78 (impl)
macro_rules! Depcrateimpl_78 {
() => {
// Module: crate
// Provides: {"impl_78"}
// Dependencies: {}
impl < T : FloatCore + Product > Product for OrderedFloat < T > { fn product < I : Iterator < Item = OrderedFloat < T > > > (iter : I) -> Self { OrderedFloat (iter . map (| v | v . 0) . product ()) } }
};
}
