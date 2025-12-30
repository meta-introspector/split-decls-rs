// Generated macro for impl_497 (impl)
macro_rules! Depcrate_readerimpl_497 {
() => {
// Module: crate::reader
// Provides: {"impl_497"}
// Dependencies: {}
impl < 'r , R > io :: BufRead for BinaryStream < 'r , R > where R : io :: BufRead , { # [inline] fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . inner . fill_buf () } # [inline] fn consume (& mut self , amt : usize) { self . inner . consume (amt) ; * self . offset += amt as u64 ; } }
};
}
