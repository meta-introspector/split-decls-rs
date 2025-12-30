// Generated macro for impl_1283 (impl)
macro_rules! Depcrate_matrix_graphimpl_1283 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1283"}
// Dependencies: {}
impl < T > Nullable for Option < T > { type Wrapped = T ; fn new (value : T) -> Self { Some (value) } fn as_ref (& self) -> Option < & Self :: Wrapped > { self . as_ref () } fn as_mut (& mut self) -> Option < & mut Self :: Wrapped > { self . as_mut () } }
};
}
