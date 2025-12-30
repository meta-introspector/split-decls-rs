// Generated macro for impl_980 (impl)
macro_rules! Depcrate_resolver_utils_containerimpl_980 {
() => {
// Module: crate::resolver_utils::container
// Provides: {"impl_980"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : ContainerType , E : Into < Error > + Send + Sync + Clone > ContainerType for Result < T , E > { async fn resolve_field (& self , ctx : & Context < '_ >) -> ServerResult < Option < Value > > { match self { Ok (value) => T :: resolve_field (value , ctx) . await , Err (err) => Err (ctx . set_error_path (err . clone () . into () . into_server_error (ctx . item . pos))) , } } async fn find_entity (& self , ctx : & Context < '_ > , params : & Value) -> ServerResult < Option < Value > > { match self { Ok (value) => T :: find_entity (value , ctx , params) . await , Err (err) => Err (ctx . set_error_path (err . clone () . into () . into_server_error (ctx . item . pos))) , } } }
};
}
