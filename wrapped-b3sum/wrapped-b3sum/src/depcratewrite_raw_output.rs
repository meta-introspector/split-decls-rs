// Generated macro for write_raw_output (function)
macro_rules! Depcratewrite_raw_output {
() => {
// Module: crate
// Provides: {"write_raw_output"}
// Dependencies: {}
fn write_raw_output (output : blake3 :: OutputReader , args : & Args) -> anyhow :: Result < () > { let mut output = output . take (args . len ()) ; let stdout = std :: io :: stdout () ; let mut handler = stdout . lock () ; std :: io :: copy (& mut output , & mut handler) ? ; Ok (()) }
};
}
