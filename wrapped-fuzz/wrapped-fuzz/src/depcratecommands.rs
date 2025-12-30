// Generated macro for Commands (enum)
macro_rules! DepcrateCommands {
() => {
// Module: crate
// Provides: {"Commands"}
// Dependencies: {}
# [derive (Clone , Subcommand , Debug)] enum Commands { # [doc = " Decode fuzzing in/out testcases (binary serialized `FuzzOp`s)"] Decode { files : Vec < PathBuf > } , # [doc = " Exhaustively test all possible ops and inputs (for 8-bit formats, by default)"] Bruteforce { # [doc = " Minimum bit-width of floating-point format to test"] # [arg (long , default_value_t = 0)] min_width : usize , # [doc = " Maximum bit-width of floating-point format to test"] # [arg (long , default_value_t = 8)] max_width : usize , # [doc = " Show failures as they happen (useful for larger formats)"] # [arg (short , long)] verbose : bool , # [doc = " Limit testing to FMA ops, and only non-trivial ones (addend != 0.0)"] # [arg (long)] only_non_trivial_fma : bool , } , }
};
}
