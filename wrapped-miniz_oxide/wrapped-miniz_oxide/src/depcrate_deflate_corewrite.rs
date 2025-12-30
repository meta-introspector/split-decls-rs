// Generated macro for write (function)
macro_rules! Depcrate_deflate_corewrite {
() => {
// Module: crate::deflate::core
// Provides: {"write"}
// Dependencies: {}
fn write (src : & [u8] , dst : & mut [u8] , dst_pos : & mut usize) -> Result < () > { match dst . get_mut (* dst_pos .. * dst_pos + src . len ()) { Some (s) => s . copy_from_slice (src) , None => return Err (Error { }) , } * dst_pos += src . len () ; Ok (()) }
};
}
