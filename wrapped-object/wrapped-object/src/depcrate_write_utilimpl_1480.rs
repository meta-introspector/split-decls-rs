// Generated macro for impl_1480 (impl)
macro_rules! Depcrate_write_utilimpl_1480 {
() => {
// Module: crate::write::util
// Provides: {"impl_1480"}
// Dependencies: {}
impl < 'a > dyn WritableBuffer + 'a { # [doc = " Writes the specified `Pod` type at the end of the buffer."] pub fn write < T : Pod > (& mut self , val : & T) { self . write_bytes (bytes_of (val)) } # [doc = " Writes the specified `Pod` slice at the end of the buffer."] pub fn write_slice < T : Pod > (& mut self , val : & [T]) { self . write_bytes (bytes_of_slice (val)) } }
};
}
