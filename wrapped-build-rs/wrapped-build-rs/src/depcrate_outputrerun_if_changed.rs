// Generated macro for rerun_if_changed (function)
macro_rules! Depcrate_outputrerun_if_changed {
() => {
// Module: crate::output
// Provides: {"rerun_if_changed"}
// Dependencies: {}
# [doc = " The `rerun-if-changed` instruction tells Cargo to re-run the build script if the"] # [doc = " file at the given path has changed."] # [doc = ""] # [doc = " Currently, Cargo only uses the filesystem"] # [doc = " last-modified “mtime” timestamp to determine if the file has changed. It"] # [doc = " compares against an internal cached timestamp of when the build script last ran."] # [doc = ""] # [doc = " If the path points to a directory, it will scan the entire directory for any"] # [doc = " modifications."] # [doc = ""] # [doc = " If the build script inherently does not need to re-run under any circumstance,"] # [doc = " then calling `rerun_if_changed(\"build.rs\")` is a simple way to prevent it from"] # [doc = " being re-run (otherwise, the default if no `rerun-if` instructions are emitted"] # [doc = " is to scan the entire package directory for changes). Cargo automatically"] # [doc = " handles whether or not the script itself needs to be recompiled, and of course"] # [doc = " the script will be re-run after it has been recompiled. Otherwise, specifying"] # [doc = " `build.rs` is redundant and unnecessary."] # [track_caller] pub fn rerun_if_changed (path : impl AsRef < Path >) { let Some (path) = path . as_ref () . to_str () else { panic ! ("cannot emit rerun-if-changed: path is not UTF-8") ; } ; if path . contains ('\n') { panic ! ("cannot emit rerun-if-changed: path contains newline") ; } emit ("rerun-if-changed" , path) ; }
};
}
