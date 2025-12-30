// Generated macro for impl_700 (impl)
macro_rules! Depcrate_frame_reasonimpl_700 {
() => {
// Module: crate::frame::reason
// Provides: {"impl_700"}
// Dependencies: {}
impl fmt :: Debug for Reason { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let name = match self . 0 { 0 => "NO_ERROR" , 1 => "PROTOCOL_ERROR" , 2 => "INTERNAL_ERROR" , 3 => "FLOW_CONTROL_ERROR" , 4 => "SETTINGS_TIMEOUT" , 5 => "STREAM_CLOSED" , 6 => "FRAME_SIZE_ERROR" , 7 => "REFUSED_STREAM" , 8 => "CANCEL" , 9 => "COMPRESSION_ERROR" , 10 => "CONNECT_ERROR" , 11 => "ENHANCE_YOUR_CALM" , 12 => "INADEQUATE_SECURITY" , 13 => "HTTP_1_1_REQUIRED" , other => return f . debug_tuple ("Reason") . field (& Hex (other)) . finish () , } ; f . write_str (name) } }
};
}
