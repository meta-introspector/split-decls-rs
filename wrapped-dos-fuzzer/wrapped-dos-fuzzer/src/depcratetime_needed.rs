// Generated macro for time_needed (function)
macro_rules! Depcratetime_needed {
() => {
// Module: crate
// Provides: {"time_needed"}
// Dependencies: {}
# [doc = " Returns the length of the tested substring and the time needed for parsing given string."] fn time_needed (sample : & str) -> Duration { let clock = Clock :: < ThreadCpuTime > :: now () ; let parser = Parser :: new_ext (sample , Options :: all ()) ; parser . for_each (| evt | { black_box :: black_box (evt) ; }) ; clock . elapsed () }
};
}
