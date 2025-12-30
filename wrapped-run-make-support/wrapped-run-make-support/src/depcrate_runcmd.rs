// Generated macro for cmd (function)
macro_rules! Depcrate_runcmd {
() => {
// Module: crate::run
// Provides: {"cmd"}
// Dependencies: {}
# [doc = " Create a new custom [`Command`]. This should be preferred to creating [`std::process::Command`]"] # [doc = " directly."] # [track_caller] pub fn cmd < S : AsRef < OsStr > > (program : S) -> Command { let mut command = Command :: new (program) ; command . env ("LC_ALL" , "C") ; command }
};
}
