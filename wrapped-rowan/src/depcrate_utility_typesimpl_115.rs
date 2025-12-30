// Generated macro for impl_115 (impl)
macro_rules! Depcrate_utility_typesimpl_115 {
() => {
// Module: crate::utility_types
// Provides: {"impl_115"}
// Dependencies: {}
impl < N , T > NodeOrToken < N , T > { pub fn into_node (self) -> Option < N > { match self { NodeOrToken :: Node (node) => Some (node) , NodeOrToken :: Token (_) => None , } } pub fn into_token (self) -> Option < T > { match self { NodeOrToken :: Node (_) => None , NodeOrToken :: Token (token) => Some (token) , } } pub fn as_node (& self) -> Option < & N > { match self { NodeOrToken :: Node (node) => Some (node) , NodeOrToken :: Token (_) => None , } } pub fn as_token (& self) -> Option < & T > { match self { NodeOrToken :: Node (_) => None , NodeOrToken :: Token (token) => Some (token) , } } }
};
}
