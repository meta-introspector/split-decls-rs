// Generated macro for impl_45 (impl)
macro_rules! Depcrate_framesimpl_45 {
() => {
// Module: crate::frames
// Provides: {"impl_45"}
// Dependencies: {}
impl Mock < frame :: Reset > { pub fn protocol_error (self) -> Self { let id = self . 0 . stream_id () ; Mock (frame :: Reset :: new (id , frame :: Reason :: PROTOCOL_ERROR)) } pub fn flow_control (self) -> Self { let id = self . 0 . stream_id () ; Mock (frame :: Reset :: new (id , frame :: Reason :: FLOW_CONTROL_ERROR)) } pub fn refused (self) -> Self { let id = self . 0 . stream_id () ; Mock (frame :: Reset :: new (id , frame :: Reason :: REFUSED_STREAM)) } pub fn cancel (self) -> Self { let id = self . 0 . stream_id () ; Mock (frame :: Reset :: new (id , frame :: Reason :: CANCEL)) } pub fn stream_closed (self) -> Self { let id = self . 0 . stream_id () ; Mock (frame :: Reset :: new (id , frame :: Reason :: STREAM_CLOSED)) } pub fn internal_error (self) -> Self { let id = self . 0 . stream_id () ; Mock (frame :: Reset :: new (id , frame :: Reason :: INTERNAL_ERROR)) } pub fn reason (self , reason : frame :: Reason) -> Self { let id = self . 0 . stream_id () ; Mock (frame :: Reset :: new (id , reason)) } }
};
}
