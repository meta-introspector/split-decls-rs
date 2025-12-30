// Generated macro for impl_651 (impl)
macro_rules! Depcrate_utilsimpl_651 {
() => {
// Module: crate::utils
// Provides: {"impl_651"}
// Dependencies: {}
impl < 'a > io :: BufRead for Fountain < 'a > { # [inline] fn fill_buf (& mut self) -> io :: Result < & [u8] > { Ok (& self . chunk [self . consumed ..]) } fn consume (& mut self , amt : usize) { self . consumed += amt ; if self . consumed == self . chunk . len () { self . consumed = 0 ; } self . overall_read += amt as u64 ; } }
};
}
