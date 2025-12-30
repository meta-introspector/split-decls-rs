// Generated macro for read_stdin (function)
macro_rules! Depcrate_cliread_stdin {
() => {
// Module: crate::cli
// Provides: {"read_stdin"}
// Dependencies: {}
fn read_stdin () -> anyhow :: Result < String > { let mut buff = String :: new () ; std :: io :: stdin () . read_to_string (& mut buff) ? ; Ok (buff) }
};
}
