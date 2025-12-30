// Generated macro for impl_151 (impl)
macro_rules! Depcrate_os_iocpimpl_151 {
() => {
// Module: crate::os::iocp
// Provides: {"impl_151"}
// Dependencies: {}
impl CompletionPacket { # [doc = " Create a new completion packet with a custom event."] pub fn new (event : Event) -> Self { Self (Arc :: pin (IoStatusBlock :: from (PacketInner :: Custom { event }))) } # [doc = " Get the event associated with this packet."] pub fn event (& self) -> & Event { let data = self . 0 . as_ref () . data () . project_ref () ; match data { PacketInnerProj :: Custom { event } => event , _ => unreachable ! () , } } }
};
}
