// Generated macro for Packet (type)
macro_rules! Depcrate_os_iocpPacket {
() => {
// Module: crate::os::iocp
// Provides: {"Packet"}
// Dependencies: {}
# [doc = " The type of our completion packet."] # [doc = ""] # [doc = " It needs to be pinned, since it contains data that is expected by IOCP not to be moved."] type Packet = Pin < Arc < PacketUnwrapped > > ;
};
}
