// Generated macro for StockVectorView (struct)
macro_rules! Depcrate_vector_viewStockVectorView {
() => {
// Module: crate::vector_view
// Provides: {"StockVectorView"}
// Dependencies: {}
# [implement (IVectorView < T >, IIterable < T >)] struct StockVectorView < T > where T : RuntimeType + 'static , T :: Default : Clone + PartialEq , { values : Vec < T :: Default > , }
};
}
