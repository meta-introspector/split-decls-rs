// Generated macro for Written (struct)
macro_rules! Depcrate_connection_streams_sendWritten {
() => {
// Module: crate::connection::streams::send
// Provides: {"Written"}
// Dependencies: {}
# [doc = " Indicates how many bytes and chunks had been transferred in a write operation"] # [derive (Debug , Default , PartialEq , Eq , Clone , Copy)] pub struct Written { # [doc = " The amount of bytes which had been written"] pub bytes : usize , # [doc = " The amount of full chunks which had been written"] # [doc = ""] # [doc = " If a chunk was only partially written, it will not be counted by this field."] pub chunks : usize , }
};
}
