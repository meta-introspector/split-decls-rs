// Generated macro for is_ascii (function)
macro_rules! Depcrate_memis_ascii {
() => {
// Module: crate::mem
// Provides: {"is_ascii"}
// Dependencies: {}
# [doc = " Checks whether the buffer is all-ASCII."] # [doc = ""] # [doc = " May read the entire buffer even if it isn't all-ASCII. (I.e. the function"] # [doc = " is not guaranteed to fail fast.)"] pub fn is_ascii (buffer : & [u8]) -> bool { is_ascii_impl (buffer) }
};
}
