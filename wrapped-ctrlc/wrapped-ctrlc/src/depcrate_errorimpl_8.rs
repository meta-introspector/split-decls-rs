// Generated macro for impl_8 (impl)
macro_rules! Depcrate_errorimpl_8 {
() => {
// Module: crate::error
// Provides: {"impl_8"}
// Dependencies: {}
impl std :: error :: Error for Error { fn description (& self) -> & str { self . describe () } fn cause (& self) -> Option < & dyn std :: error :: Error > { match * self { Error :: System (ref e) => Some (e) , _ => None , } } }
};
}
