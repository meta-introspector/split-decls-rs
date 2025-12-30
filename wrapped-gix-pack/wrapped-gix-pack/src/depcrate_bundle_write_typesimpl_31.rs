// Generated macro for impl_31 (impl)
macro_rules! Depcrate_bundle_write_typesimpl_31 {
() => {
// Module: crate::bundle::write::types
// Provides: {"impl_31"}
// Dependencies: {}
impl < R > io :: BufRead for PassThrough < R > where R : io :: BufRead , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . reader . fill_buf () } fn consume (& mut self , amt : usize) { self . reader . consume (amt) ; } }
};
}
