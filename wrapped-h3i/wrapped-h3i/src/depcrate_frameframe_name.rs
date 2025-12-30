// Generated macro for frame_name (function)
macro_rules! Depcrate_frameframe_name {
() => {
// Module: crate::frame
// Provides: {"frame_name"}
// Dependencies: {}
fn frame_name (frame : & QFrame) -> & 'static str { match frame { QFrame :: Data { .. } => "DATA" , QFrame :: Headers { .. } => "HEADERS" , QFrame :: CancelPush { .. } => "CANCEL_PUSH" , QFrame :: Settings { .. } => "SETTINGS" , QFrame :: PushPromise { .. } => "PUSH_PROMISE" , QFrame :: GoAway { .. } => "GO_AWAY" , QFrame :: MaxPushId { .. } => "MAX_PUSH_ID" , QFrame :: PriorityUpdateRequest { .. } => "PRIORITY_UPDATE(REQUEST)" , QFrame :: PriorityUpdatePush { .. } => "PRIORITY_UPDATE(PUSH)" , QFrame :: Unknown { .. } => "UNKNOWN" , } }
};
}
