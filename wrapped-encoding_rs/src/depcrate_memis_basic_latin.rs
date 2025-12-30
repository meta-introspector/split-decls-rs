// Generated macro for is_basic_latin (function)
macro_rules! Depcrate_memis_basic_latin {
() => {
// Module: crate::mem
// Provides: {"is_basic_latin"}
// Dependencies: {}
# [doc = " Checks whether the buffer is all-Basic Latin (i.e. UTF-16 representing"] # [doc = " only ASCII characters)."] # [doc = ""] # [doc = " May read the entire buffer even if it isn't all-ASCII. (I.e. the function"] # [doc = " is not guaranteed to fail fast.)"] pub fn is_basic_latin (buffer : & [u16]) -> bool { is_basic_latin_impl (buffer) }
};
}
