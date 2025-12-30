// Generated macro for impl_1630 (impl)
macro_rules! Depcrate_streamimpl_1630 {
() => {
// Module: crate::stream
// Provides: {"impl_1630"}
// Dependencies: {}
impl std :: hash :: Hasher for StreamIdHasher { # [inline] fn finish (& self) -> u64 { self . id } # [inline] fn write_u64 (& mut self , id : u64) { self . id = id ; } # [inline] fn write (& mut self , _ : & [u8]) { unimplemented ! () } }
};
}
