// Generated macro for impl_73 (impl)
macro_rules! Depcrate_unix_termimpl_73 {
() => {
// Module: crate::unix_term
// Provides: {"impl_73"}
// Dependencies: {}
impl Input < BufReader < fs :: File > > { fn buffered () -> io :: Result < Self > { Ok (match Input :: unbuffered () ? { Input :: Stdin (s) => Input :: Stdin (s) , Input :: File (f) => Input :: File (BufReader :: new (f)) , }) } }
};
}
