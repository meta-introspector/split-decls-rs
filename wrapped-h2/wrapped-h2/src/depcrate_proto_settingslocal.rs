// Generated macro for Local (enum)
macro_rules! Depcrate_proto_settingsLocal {
() => {
// Module: crate::proto::settings
// Provides: {"Local"}
// Dependencies: {}
# [derive (Debug)] enum Local { # [doc = " We want to send these SETTINGS to the remote when the socket is ready."] ToSend (frame :: Settings) , # [doc = " We have sent these SETTINGS and are waiting for the remote to ACK"] # [doc = " before we apply them."] WaitingAck (frame :: Settings) , # [doc = " Our local settings are in sync with the remote."] Synced , }
};
}
