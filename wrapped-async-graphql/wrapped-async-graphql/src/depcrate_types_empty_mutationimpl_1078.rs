// Generated macro for impl_1078 (impl)
macro_rules! Depcrate_types_empty_mutationimpl_1078 {
() => {
// Module: crate::types::empty_mutation
// Provides: {"impl_1078"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl ContainerType for EmptyMutation { fn is_empty () -> bool { true } async fn resolve_field (& self , _ctx : & Context < '_ >) -> ServerResult < Option < Value > > { Ok (None) } }
};
}
