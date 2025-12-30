// Generated macro for impl_99 (impl)
macro_rules! Depcrate_progressimpl_99 {
() => {
// Module: crate::progress
// Provides: {"impl_99"}
// Dependencies: {}
impl < T , P > io :: BufRead for Read < T , P > where T : io :: BufRead , P : Progress , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . inner . fill_buf () } fn consume (& mut self , amt : usize) { self . inner . consume (amt) ; } }
};
}
