// Generated macro for impl_116 (impl)
macro_rules! Depcrate_utility_typesimpl_116 {
() => {
// Module: crate::utility_types
// Provides: {"impl_116"}
// Dependencies: {}
impl < N : Deref , T : Deref > NodeOrToken < N , T > { pub (crate) fn as_deref (& self) -> NodeOrToken < & N :: Target , & T :: Target > { match self { NodeOrToken :: Node (node) => NodeOrToken :: Node (node) , NodeOrToken :: Token (token) => NodeOrToken :: Token (token) , } } }
};
}
