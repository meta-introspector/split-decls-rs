// Generated macro for is_self (function)
macro_rules! Depcrate_resolvingis_self {
() => {
// Module: crate::resolving
// Provides: {"is_self"}
// Dependencies: {}
fn is_self (path : & ast :: Path) -> bool { path . segment () . map (| segment | segment . self_token () . is_some ()) . unwrap_or (false) }
};
}
