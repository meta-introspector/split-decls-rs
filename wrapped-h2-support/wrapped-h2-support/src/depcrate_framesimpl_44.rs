// Generated macro for impl_44 (impl)
macro_rules! Depcrate_framesimpl_44 {
() => {
// Module: crate::frames
// Provides: {"impl_44"}
// Dependencies: {}
impl Mock < frame :: GoAway > { pub fn protocol_error (self) -> Self { self . reason (frame :: Reason :: PROTOCOL_ERROR) } pub fn internal_error (self) -> Self { self . reason (frame :: Reason :: INTERNAL_ERROR) } pub fn flow_control (self) -> Self { self . reason (frame :: Reason :: FLOW_CONTROL_ERROR) } pub fn frame_size (self) -> Self { self . reason (frame :: Reason :: FRAME_SIZE_ERROR) } pub fn calm (self) -> Self { self . reason (frame :: Reason :: ENHANCE_YOUR_CALM) } pub fn no_error (self) -> Self { self . reason (frame :: Reason :: NO_ERROR) } pub fn data < I > (self , debug_data : I) -> Self where I : Into < Bytes > , { Mock (frame :: GoAway :: with_debug_data (self . 0 . last_stream_id () , self . 0 . reason () , debug_data . into () ,)) } pub fn reason (self , reason : frame :: Reason) -> Self { Mock (frame :: GoAway :: with_debug_data (self . 0 . last_stream_id () , reason , self . 0 . debug_data () . clone () ,)) } }
};
}
