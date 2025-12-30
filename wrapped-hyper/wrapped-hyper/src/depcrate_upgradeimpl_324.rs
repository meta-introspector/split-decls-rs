// Generated macro for impl_324 (impl)
macro_rules! Depcrate_upgradeimpl_324 {
() => {
// Module: crate::upgrade
// Provides: {"impl_324"}
// Dependencies: {}
impl Read for Upgraded { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : ReadBufCursor < '_ > ,) -> Poll < io :: Result < () > > { Pin :: new (& mut self . io) . poll_read (cx , buf) } }
};
}
