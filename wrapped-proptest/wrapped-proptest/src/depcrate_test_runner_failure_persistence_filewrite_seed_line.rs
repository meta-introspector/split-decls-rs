// Generated macro for write_seed_line (function)
macro_rules! Depcrate_test_runner_failure_persistence_filewrite_seed_line {
() => {
// Module: crate::test_runner::failure_persistence::file
// Provides: {"write_seed_line"}
// Dependencies: {}
fn write_seed_line (buf : & mut Vec < u8 > , seed : & PersistedSeed , shrunken_value : & dyn Debug ,) -> io :: Result < () > { write ! (buf , "{}" , seed . to_string ()) ? ; let debug_start = buf . len () ; write ! (buf , " # shrinks to {:?}" , shrunken_value) ? ; for byte in & mut buf [debug_start ..] { if b'\n' == * byte || b'\r' == * byte { * byte = b' ' ; } } buf . push (b'\n') ; Ok (()) }
};
}
