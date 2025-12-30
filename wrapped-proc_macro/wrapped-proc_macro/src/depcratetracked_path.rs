// Generated macro for tracked_path (module)
macro_rules! Depcratetracked_path {
() => {
// Module: crate
// Provides: {"tracked_path"}
// Dependencies: {}
# [doc = " Tracked access to additional files."] # [unstable (feature = "track_path" , issue = "99515")] pub mod tracked_path { # [doc = " Track a file explicitly."] # [doc = ""] # [doc = " Commonly used for tracking asset preprocessing."] # [unstable (feature = "track_path" , issue = "99515")] pub fn path < P : AsRef < str > > (path : P) { let path : & str = path . as_ref () ; crate :: bridge :: client :: FreeFunctions :: track_path (path) ; } }
};
}
