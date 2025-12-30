// Generated macro for impl_207 (impl)
macro_rules! Depcrate_schemaimpl_207 {
() => {
// Module: crate::schema
// Provides: {"impl_207"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshSchema for std :: net :: Ipv6Addr { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { < ip_addr_std_derive_impl :: Ipv6Addr > :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { ip_addr_std_derive_impl :: Ipv6Addr :: declaration () } }
};
}
