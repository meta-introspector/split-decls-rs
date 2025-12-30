// Generated macro for impl_117 (impl)
macro_rules! Depcrate_utility_typesimpl_117 {
() => {
// Module: crate::utility_types
// Provides: {"impl_117"}
// Dependencies: {}
impl < N : fmt :: Display , T : fmt :: Display > fmt :: Display for NodeOrToken < N , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { NodeOrToken :: Node (node) => fmt :: Display :: fmt (node , f) , NodeOrToken :: Token (token) => fmt :: Display :: fmt (token , f) , } } }
};
}
