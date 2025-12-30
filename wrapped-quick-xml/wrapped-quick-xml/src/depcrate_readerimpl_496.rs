// Generated macro for impl_496 (impl)
macro_rules! Depcrate_readerimpl_496 {
() => {
// Module: crate::reader
// Provides: {"impl_496"}
// Dependencies: {}
impl < 'r , R > io :: Read for BinaryStream < 'r , R > where R : io :: Read , { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let amt = self . inner . read (buf) ? ; * self . offset += amt as u64 ; Ok (amt) } }
};
}
