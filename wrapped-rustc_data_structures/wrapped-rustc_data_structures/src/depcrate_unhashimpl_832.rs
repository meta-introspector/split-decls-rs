// Generated macro for impl_832 (impl)
macro_rules! Depcrate_unhashimpl_832 {
() => {
// Module: crate::unhash
// Provides: {"impl_832"}
// Dependencies: {}
impl Hasher for Unhasher { # [inline] fn finish (& self) -> u64 { self . value } fn write (& mut self , _bytes : & [u8]) { unimplemented ! ("use write_u64") ; } # [inline] fn write_u64 (& mut self , value : u64) { debug_assert_eq ! (0 , self . value , "Unhasher doesn't mix values!") ; self . value = value ; } }
};
}
