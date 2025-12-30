// Generated macro for impl_262 (impl)
macro_rules! Depcrate_bytes_mutimpl_262 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_262"}
// Dependencies: {}
impl fmt :: Write for BytesMut { # [inline] fn write_str (& mut self , s : & str) -> fmt :: Result { if self . remaining_mut () >= s . len () { self . put_slice (s . as_bytes ()) ; Ok (()) } else { Err (fmt :: Error) } } # [inline] fn write_fmt (& mut self , args : fmt :: Arguments < '_ >) -> fmt :: Result { fmt :: write (self , args) } }
};
}
