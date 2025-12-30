// Generated macro for impl_75 (impl)
macro_rules! Depcrate_unix_termimpl_75 {
() => {
// Module: crate::unix_term
// Provides: {"impl_75"}
// Dependencies: {}
impl < T : BufRead > Input < T > { fn read_line (& mut self , buf : & mut String) -> io :: Result < usize > { match self { Self :: Stdin (s) => s . read_line (buf) , Self :: File (f) => f . read_line (buf) , } } }
};
}
