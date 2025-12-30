// Generated macro for impl_67 (impl)
macro_rules! Depcrate_shutdownimpl_67 {
() => {
// Module: crate::shutdown
// Provides: {"impl_67"}
// Dependencies: {}
# [cfg (unix)] impl ShutdownConnection for UnixDatagram { # [inline] fn poll_shutdown (& mut self , _cx : & mut Context) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}
