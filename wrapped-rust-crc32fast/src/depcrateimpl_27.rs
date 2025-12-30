// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl hash :: Hasher for Hasher { fn write (& mut self , bytes : & [u8]) { self . update (bytes) } fn finish (& self) -> u64 { u64 :: from (self . clone () . finalize ()) } }
};
}
