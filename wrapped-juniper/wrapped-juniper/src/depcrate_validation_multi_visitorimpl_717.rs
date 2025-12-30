// Generated macro for impl_717 (impl)
macro_rules! Depcrate_validation_multi_visitorimpl_717 {
() => {
// Module: crate::validation::multi_visitor
// Provides: {"impl_717"}
// Dependencies: {}
impl < A , B > MultiVisitorCons < A , B > { pub fn with < V > (self , visitor : V) -> MultiVisitorCons < V , Self > { MultiVisitorCons (visitor , self) } }
};
}
