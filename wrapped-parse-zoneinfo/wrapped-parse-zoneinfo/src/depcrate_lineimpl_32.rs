// Generated macro for impl_32 (impl)
macro_rules! Depcrate_lineimpl_32 {
() => {
// Module: crate::line
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a > Saving < 'a > { fn from_str (input : & 'a str) -> Result < Self , Error > { if input == "-" { Ok (Self :: NoSaving) } else if input . chars () . all (| c | c == '-' || c == '_' || c . is_alphabetic ()) { Ok (Self :: Multiple (input)) } else if let Ok (time) = TimeSpec :: from_str (input) { Ok (Self :: OneOff (time)) } else { Err (Error :: CouldNotParseSaving (input . to_string ())) } } }
};
}
