// Generated macro for impl_29 (impl)
macro_rules! Depcrate_interruptimpl_29 {
() => {
// Module: crate::interrupt
// Provides: {"impl_29"}
// Dependencies: {}
impl < R > io :: BufRead for Read < '_ , R > where R : io :: BufRead , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . inner . fill_buf () } fn consume (& mut self , amt : usize) { self . inner . consume (amt) ; } }
};
}
