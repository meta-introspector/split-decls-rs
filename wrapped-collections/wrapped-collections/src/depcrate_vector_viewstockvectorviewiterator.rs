// Generated macro for StockVectorViewIterator (struct)
macro_rules! Depcrate_vector_viewStockVectorViewIterator {
() => {
// Module: crate::vector_view
// Provides: {"StockVectorViewIterator"}
// Dependencies: {}
# [implement (IIterator < T >)] struct StockVectorViewIterator < T > where T : RuntimeType + 'static , T :: Default : Clone + PartialEq , { owner : ComObject < StockVectorView < T > > , current : std :: sync :: atomic :: AtomicUsize , }
};
}
