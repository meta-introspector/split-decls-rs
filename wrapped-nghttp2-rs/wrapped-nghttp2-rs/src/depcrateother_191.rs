// Generated macro for other_191 (other)
macro_rules! Depcrateother_191 {
() => {
// Module: crate
// Provides: {"other_191"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " This option sets the maximum dynamic table size for deflating"] # [doc = " header fields.  The default value is 4KiB.  In HTTP/2, receiver of"] # [doc = " deflated header block can specify maximum dynamic table size.  The"] # [doc = " actual maximum size is the minimum of the size receiver specified"] # [doc = " and this option value."] pub fn nghttp2_option_set_max_deflate_dynamic_table_size (option : * mut nghttp2_option , val : usize ,) ; }
};
}
