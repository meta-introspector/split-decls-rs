// Generated macro for resolve_container_serial (function)
macro_rules! Depcrate_resolver_utils_containerresolve_container_serial {
() => {
// Module: crate::resolver_utils::container
// Provides: {"resolve_container_serial"}
// Dependencies: {}
# [doc = " Resolve an container by executing each of the fields serially."] pub async fn resolve_container_serial < 'a , T : ContainerType + ? Sized > (ctx : & ContextSelectionSet < 'a > , root : & 'a T ,) -> ServerResult < Value > { resolve_container_inner (ctx , root , false) . await }
};
}
