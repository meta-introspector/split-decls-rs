// Generated macro for provide (function)
macro_rules! Depcrateprovide {
() => {
// Module: crate
// Provides: {"provide"}
// Dependencies: {}
# [allow (missing_docs)] pub fn provide (providers : & mut Providers) { providers . hooks . save_dep_graph = | tcx | tcx . sess . time ("serialize_dep_graph" , | | persist :: save_dep_graph (tcx)) ; }
};
}
