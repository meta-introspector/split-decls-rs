// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl Read for CurlSubtransport { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if self . reader . is_none () { self . execute (& []) ? ; } self . reader . as_mut () . unwrap () . read (buf) } }
};
}
