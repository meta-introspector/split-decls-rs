// Generated macro for impl_68 (impl)
macro_rules! Depcrate_shutdownimpl_68 {
() => {
// Module: crate::shutdown
// Provides: {"impl_68"}
// Dependencies: {}
impl < T : ShutdownConnection + Send + Sync > ShutdownConnection for Arc < T > { # [inline] fn poll_shutdown (& mut self , _cx : & mut Context) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}
