// Generated macro for impl_208 (impl)
macro_rules! Depcrate_schemaimpl_208 {
() => {
// Module: crate::schema
// Provides: {"impl_208"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshSchema for std :: net :: IpAddr { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { < ip_addr_std_derive_impl :: IpAddr > :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { ip_addr_std_derive_impl :: IpAddr :: declaration () } }
};
}
