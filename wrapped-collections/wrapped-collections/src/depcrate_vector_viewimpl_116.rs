// Generated macro for impl_116 (impl)
macro_rules! Depcrate_vector_viewimpl_116 {
() => {
// Module: crate::vector_view
// Provides: {"impl_116"}
// Dependencies: {}
impl < T > From < Vec < T :: Default > > for IVectorView < T > where T : RuntimeType , T :: Default : Clone + PartialEq , { fn from (values : Vec < T :: Default >) -> Self { ComObject :: new (StockVectorView { values }) . into_interface () } }
};
}
