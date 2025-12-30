// Generated macro for impl_98 (impl)
macro_rules! Depcrate_fmtimpl_98 {
() => {
// Module: crate::fmt
// Provides: {"impl_98"}
// Dependencies: {}
impl Write for Formatter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . buf . borrow_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . buf . borrow_mut () . flush () } }
};
}
