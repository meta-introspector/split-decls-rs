// Generated macro for impl_206 (impl)
macro_rules! Depcrate_schemaimpl_206 {
() => {
// Module: crate::schema
// Provides: {"impl_206"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshSchema for std :: net :: Ipv4Addr { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { < ip_addr_std_derive_impl :: Ipv4Addr > :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { ip_addr_std_derive_impl :: Ipv4Addr :: declaration () } }
};
}
