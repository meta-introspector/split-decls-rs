// Generated macro for debug_list_bytes (function)
macro_rules! Depcrate_read_utildebug_list_bytes {
() => {
// Module: crate::read::util
// Provides: {"debug_list_bytes"}
// Dependencies: {}
fn debug_list_bytes (bytes : & [u8] , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut list = fmt . debug_list () ; list . entries (bytes . iter () . take (8) . copied () . map (DebugByte)) ; if bytes . len () > 8 { list . entry (& DebugLen (bytes . len ())) ; } list . finish () }
};
}
