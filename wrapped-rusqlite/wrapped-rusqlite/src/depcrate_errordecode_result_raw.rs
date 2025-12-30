// Generated macro for decode_result_raw (function)
macro_rules! Depcrate_errordecode_result_raw {
() => {
// Module: crate::error
// Provides: {"decode_result_raw"}
// Dependencies: {}
pub unsafe fn decode_result_raw (db : * mut ffi :: sqlite3 , code : c_int) -> Result < () > { if code == ffi :: SQLITE_OK { Ok (()) } else { Err (error_from_handle (db , code)) } }
};
}
