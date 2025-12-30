// Generated macro for impl_1336 (impl)
macro_rules! Depcrate_remote_connection_fetchimpl_1336 {
() => {
// Module: crate::remote::connection::fetch
// Provides: {"impl_1336"}
// Dependencies: {}
impl < T > Prepare < '_ , '_ , T > where T : Transport , { # [doc = " Return the `ref_map` (that includes the server handshake) which was part of listing refs prior to fetching a pack."] pub fn ref_map (& self) -> & RefMap { & self . ref_map } }
};
}
