// Generated macro for impl_628 (impl)
macro_rules! Depcrate_extensionsimpl_628 {
() => {
// Module: crate::extensions
// Provides: {"impl_628"}
// Dependencies: {}
impl Hasher for IdHasher { fn write (& mut self , _ : & [u8]) { unreachable ! ("TypeId calls write_u64") ; } # [inline] fn write_u64 (& mut self , id : u64) { self . 0 = id ; } # [inline] fn finish (& self) -> u64 { self . 0 } }
};
}
