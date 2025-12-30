// Generated macro for message (function)
macro_rules! Depcrate_reference_logmessage {
() => {
// Module: crate::reference::log
// Provides: {"message"}
// Dependencies: {}
# [doc = " Generate a message typical for git commit logs based on the given `operation`, commit `message` and `num_parents` of the commit."] pub fn message (operation : & str , message : & BStr , num_parents : usize) -> BString { let mut out = BString :: from (operation) ; if let Some (commit_type) = commit_type_by_parents (num_parents) { out . push_str (b" (") ; out . extend_from_slice (commit_type . as_bytes ()) ; out . push_byte (b')') ; } out . push_str (b": ") ; out . extend_from_slice (& MessageRef :: from_bytes (message) . summary ()) ; out }
};
}
