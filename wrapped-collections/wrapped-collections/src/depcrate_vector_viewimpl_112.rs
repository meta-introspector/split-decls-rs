// Generated macro for impl_112 (impl)
macro_rules! Depcrate_vector_viewimpl_112 {
() => {
// Module: crate::vector_view
// Provides: {"impl_112"}
// Dependencies: {}
impl < T > IIterable_Impl < T > for StockVectorView_Impl < T > where T : RuntimeType , T :: Default : Clone + PartialEq , { fn First (& self) -> Result < IIterator < T > > { Ok (ComObject :: new (StockVectorViewIterator { owner : self . to_object () , current : 0 . into () , }) . into_interface ()) } }
};
}
