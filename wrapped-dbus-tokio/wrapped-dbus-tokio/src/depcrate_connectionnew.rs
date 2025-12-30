// Generated macro for new (function)
macro_rules! Depcrate_connectionnew {
() => {
// Module: crate::connection
// Provides: {"new"}
// Dependencies: {}
# [doc = " Generic connection creator, you might want to use e g `new_session_local`, `new_system_sync` etc for convenience."] pub fn new < C : From < Channel > + NonblockReply > (b : BusType) -> Result < (IOResource < C > , Arc < C >) , dbus :: Error > { let channel = Channel :: get_private (b) ? ; from_channel (channel) }
};
}
