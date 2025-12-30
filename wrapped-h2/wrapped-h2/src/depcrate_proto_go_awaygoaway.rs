// Generated macro for GoAway (struct)
macro_rules! Depcrate_proto_go_awayGoAway {
() => {
// Module: crate::proto::go_away
// Provides: {"GoAway"}
// Dependencies: {}
# [doc = " Manages our sending of GOAWAY frames."] # [derive (Debug)] pub (super) struct GoAway { # [doc = " Whether the connection should close now, or wait until idle."] close_now : bool , # [doc = " Records if we've sent any GOAWAY before."] going_away : Option < GoingAway > , # [doc = " Whether the user started the GOAWAY by calling `abrupt_shutdown`."] is_user_initiated : bool , # [doc = " A GOAWAY frame that must be buffered in the Codec immediately."] pending : Option < frame :: GoAway > , }
};
}
