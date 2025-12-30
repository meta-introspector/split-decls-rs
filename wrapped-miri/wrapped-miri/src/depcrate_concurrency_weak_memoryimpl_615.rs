// Generated macro for impl_615 (impl)
macro_rules! Depcrate_concurrency_weak_memoryimpl_615 {
() => {
// Module: crate::concurrency::weak_memory
// Provides: {"impl_615"}
// Dependencies: {}
impl VisitProvenance for StoreBufferAlloc { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let Self { store_buffers } = self ; for val in store_buffers . borrow () . iter () . flat_map (| buf | buf . buffer . iter () . map (| element | & element . val)) { val . visit_provenance (visit) ; } } }
};
}
