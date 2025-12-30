// Generated macro for Outcome (struct)
macro_rules! Depcrate_walkOutcome {
() => {
// Module: crate::walk
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " Additional information collected as outcome of [`walk()`](function::walk())."] # [derive (Default , Debug , Clone , Ord , PartialOrd , Eq , PartialEq)] pub struct Outcome { # [doc = " The amount of calls to read the directory contents."] pub read_dir_calls : u32 , # [doc = " The amount of returned entries provided to the callback. This number can be lower than `seen_entries`."] pub returned_entries : usize , # [doc = " The amount of entries, prior to pathspecs filtering them out or otherwise excluding them."] pub seen_entries : u32 , }
};
}
