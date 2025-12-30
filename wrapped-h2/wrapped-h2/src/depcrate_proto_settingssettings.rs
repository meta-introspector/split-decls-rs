// Generated macro for Settings (struct)
macro_rules! Depcrate_proto_settingsSettings {
() => {
// Module: crate::proto::settings
// Provides: {"Settings"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct Settings { # [doc = " Our local SETTINGS sync state with the remote."] local : Local , # [doc = " Received SETTINGS frame pending processing. The ACK must be written to"] # [doc = " the socket first then the settings applied **before** receiving any"] # [doc = " further frames."] remote : Option < frame :: Settings > , # [doc = " Whether the connection has received the initial SETTINGS frame from the"] # [doc = " remote peer."] has_received_remote_initial_settings : bool , }
};
}
