// Generated macro for impl_619 (impl)
macro_rules! Depcrate_frame_go_awayimpl_619 {
() => {
// Module: crate::frame::go_away
// Provides: {"impl_619"}
// Dependencies: {}
impl fmt :: Debug for GoAway { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut builder = f . debug_struct ("GoAway") ; builder . field ("error_code" , & self . error_code) ; builder . field ("last_stream_id" , & self . last_stream_id) ; if ! self . debug_data . is_empty () { builder . field ("debug_data" , & self . debug_data) ; } builder . finish () } }
};
}
