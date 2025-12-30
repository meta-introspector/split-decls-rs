// Generated macro for MAX_HEADER_NAME_LEN (const)
macro_rules! Depcrate_headerMAX_HEADER_NAME_LEN {
() => {
// Module: crate::header
// Provides: {"MAX_HEADER_NAME_LEN"}
// Dependencies: {}
# [doc = " Maximum length of a header name"] # [doc = ""] # [doc = " Generally, 64kb for a header name is WAY too much than would ever be needed"] # [doc = " in practice. Restricting it to this size enables using `u16` values to"] # [doc = " represent offsets when dealing with header names."] const MAX_HEADER_NAME_LEN : usize = (1 << 16) - 1 ;
};
}
