// Generated macro for add_renamed_lints (function)
macro_rules! Depcrateadd_renamed_lints {
() => {
// Module: crate
// Provides: {"add_renamed_lints"}
// Dependencies: {}
# [doc = " Adds `Lint`s that have been renamed."] fn add_renamed_lints (lints : & mut Vec < Lint >) { for (level , names) in RENAMES { for (from , to) in * names { lints . push (Lint { name : from . to_string () , doc : vec ! [format ! ("The lint `{from}` has been renamed to [`{to}`](#{to}).")] , level : * level , path : PathBuf :: new () , lineno : 0 , }) ; } } }
};
}
