// Generated macro for GoingAway (struct)
macro_rules! Depcrate_proto_go_awayGoingAway {
() => {
// Module: crate::proto::go_away
// Provides: {"GoingAway"}
// Dependencies: {}
# [doc = " Keeps a memory of any GOAWAY frames we've sent before."] # [doc = ""] # [doc = " This looks very similar to a `frame::GoAway`, but is a separate type. Why?"] # [doc = " Mostly for documentation purposes. This type is to record status. If it"] # [doc = " were a `frame::GoAway`, it might appear like we eventually wanted to"] # [doc = " serialize it. We **only** want to be able to look up these fields at a"] # [doc = " later time."] # [derive (Debug)] pub (crate) struct GoingAway { # [doc = " Stores the highest stream ID of a GOAWAY that has been sent."] # [doc = ""] # [doc = " It's illegal to send a subsequent GOAWAY with a higher ID."] last_processed_id : StreamId , # [doc = " Records the error code of any GOAWAY frame sent."] reason : Reason , }
};
}
