// Generated macro for StockIterator (struct)
macro_rules! Depcrate_iterableStockIterator {
() => {
// Module: crate::iterable
// Provides: {"StockIterator"}
// Dependencies: {}
# [implement (IIterator < T >)] struct StockIterator < T > where T : RuntimeType + 'static , T :: Default : Clone , { owner : ComObject < StockIterable < T > > , current : std :: sync :: atomic :: AtomicUsize , }
};
}
