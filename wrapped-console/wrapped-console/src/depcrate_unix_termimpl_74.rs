// Generated macro for impl_74 (impl)
macro_rules! Depcrate_unix_termimpl_74 {
() => {
// Module: crate::unix_term
// Provides: {"impl_74"}
// Dependencies: {}
impl Input < fs :: File > { fn unbuffered () -> io :: Result < Self > { let stdin = io :: stdin () ; if is_a_terminal (& stdin) { Ok (Input :: Stdin (stdin)) } else { let f = fs :: OpenOptions :: new () . read (true) . write (true) . open ("/dev/tty") ? ; Ok (Input :: File (f)) } } }
};
}
