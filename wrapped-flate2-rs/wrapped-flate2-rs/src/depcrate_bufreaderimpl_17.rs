// Generated macro for impl_17 (impl)
macro_rules! Depcrate_bufreaderimpl_17 {
() => {
// Module: crate::bufreader
// Provides: {"impl_17"}
// Dependencies: {}
impl < R : Read > BufRead for BufReader < R > { fn fill_buf (& mut self) -> io :: Result < & [u8] > { if self . pos == self . cap { self . cap = self . inner . read (& mut self . buf) ? ; self . pos = 0 ; } Ok (& self . buf [self . pos .. self . cap]) } fn consume (& mut self , amt : usize) { self . pos = cmp :: min (self . pos + amt , self . cap) ; } }
};
}
