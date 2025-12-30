// Generated macro for impl_1286 (impl)
macro_rules! Depcrate_matrix_graphimpl_1286 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1286"}
// Dependencies: {}
impl < T : Zero > Nullable for NotZero < T > { # [doc (hidden)] type Wrapped = T ; # [doc (hidden)] fn new (value : T) -> Self { assert ! (! value . is_zero ()) ; NotZero (value) } # [doc (hidden)] fn is_null (& self) -> bool { self . 0 . is_zero () } # [doc (hidden)] fn as_ref (& self) -> Option < & Self :: Wrapped > { if ! self . is_null () { Some (& self . 0) } else { None } } # [doc (hidden)] fn as_mut (& mut self) -> Option < & mut Self :: Wrapped > { if ! self . is_null () { Some (& mut self . 0) } else { None } } }
};
}
