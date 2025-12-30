// Generated macro for macro_154 (macro)
macro_rules! Depcrate_os_iocpmacro_154 {
() => {
// Module: crate::os::iocp
// Provides: {"macro_154"}
// Dependencies: {}
pin_project ! { # [doc = " The inner type of the packet."] # [project_ref = PacketInnerProj] # [project = PacketInnerProjMut] enum PacketInner { Socket { # [pin] packet : UnsafeCell < AfdPollInfo >, socket : Mutex < SocketState > } , # [doc = " A packet for a waitable handle."] Waitable { handle : Mutex < WaitableState > } , # [doc = " A custom event sent by the user."] Custom { event : Event , } , Wakeup { # [pin] _pinned : PhantomPinned } , } }
};
}
