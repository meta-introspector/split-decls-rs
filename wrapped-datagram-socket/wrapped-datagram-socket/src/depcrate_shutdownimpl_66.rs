// Generated macro for impl_66 (impl)
macro_rules! Depcrate_shutdownimpl_66 {
() => {
// Module: crate::shutdown
// Provides: {"impl_66"}
// Dependencies: {}
impl ShutdownConnection for UdpSocket { # [inline] fn poll_shutdown (& mut self , _cx : & mut Context) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}
