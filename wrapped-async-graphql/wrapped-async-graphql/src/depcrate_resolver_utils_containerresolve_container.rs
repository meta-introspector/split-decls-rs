// Generated macro for resolve_container (function)
macro_rules! Depcrate_resolver_utils_containerresolve_container {
() => {
// Module: crate::resolver_utils::container
// Provides: {"resolve_container"}
// Dependencies: {}
# [doc = " Resolve an container by executing each of the fields concurrently."] pub async fn resolve_container < 'a , T : ContainerType + ? Sized > (ctx : & ContextSelectionSet < 'a > , root : & 'a T ,) -> ServerResult < Value > { resolve_container_inner (ctx , root , true) . await }
};
}
