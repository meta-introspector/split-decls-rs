// Generated macro for write_align (function)
macro_rules! Depcrate_write_utilwrite_align {
() => {
// Module: crate::write::util
// Provides: {"write_align"}
// Dependencies: {}
pub (crate) fn write_align (buffer : & mut dyn WritableBuffer , size : usize) { let new_len = align (buffer . len () , size) ; buffer . resize (new_len) ; }
};
}
