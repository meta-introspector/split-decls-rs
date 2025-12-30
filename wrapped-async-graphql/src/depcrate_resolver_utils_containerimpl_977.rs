// Generated macro for impl_977 (impl)
macro_rules! Depcrate_resolver_utils_containerimpl_977 {
() => {
// Module: crate::resolver_utils::container
// Provides: {"impl_977"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : ContainerType + ? Sized > ContainerType for & T { async fn resolve_field (& self , ctx : & Context < '_ >) -> ServerResult < Option < Value > > { T :: resolve_field (* self , ctx) . await } async fn find_entity (& self , ctx : & Context < '_ > , params : & Value) -> ServerResult < Option < Value > > { T :: find_entity (* self , ctx , params) . await } }
};
}
