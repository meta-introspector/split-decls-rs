// Generated macro for NodeDebugInfo (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsNodeDebugInfo {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"NodeDebugInfo"}
// Dependencies: {}
# [doc = " Some information that is irrelevant for the algorithm but very"] # [doc = " convenient to know about a tag for debugging and testing."] # [derive (Clone , Debug)] pub struct NodeDebugInfo { # [doc = " The tag in question."] pub tag : BorTag , # [doc = " Name(s) that were associated with this tag (comma-separated)."] # [doc = " Typically the name of the variable holding the corresponding"] # [doc = " pointer in the source code."] # [doc = " Helps match tag numbers to human-readable names."] pub name : Option < String > , # [doc = " Notable events in the history of this tag, used for"] # [doc = " diagnostics."] # [doc = ""] # [doc = " NOTE: by virtue of being part of `NodeDebugInfo`,"] # [doc = " the history is automatically cleaned up by the GC."] # [doc = " NOTE: this is `!Send`, it needs to be converted before displaying"] # [doc = " the actual diagnostics because `src/diagnostics.rs` requires `Send`."] pub history : History , }
};
}
