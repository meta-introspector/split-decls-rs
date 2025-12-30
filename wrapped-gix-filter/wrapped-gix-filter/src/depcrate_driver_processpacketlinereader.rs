// Generated macro for PacketlineReader (type)
macro_rules! Depcrate_driver_processPacketlineReader {
() => {
// Module: crate::driver::process
// Provides: {"PacketlineReader"}
// Dependencies: {}
type PacketlineReader < 'a , T = std :: process :: ChildStdout > = WithSidebands < 'a , T , fn (bool , & [u8]) -> gix_packetline :: read :: ProgressAction > ;
};
}
