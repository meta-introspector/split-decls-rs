// Generated macro for copy (function)
macro_rules! Depcrate_utilcopy {
() => {
// Module: crate::util
// Provides: {"copy"}
// Dependencies: {}
# [doc = " Wraps `fs::copy` with a nicer error message."] pub fn copy < P : AsRef < Path > , Q : AsRef < Path > > (from : P , to : Q) -> Result < u64 > { if fs :: symlink_metadata (& from) ? . file_type () . is_symlink () { let link = fs :: read_link (& from) ? ; symlink_file (link , & to) ? ; Ok (0) } else { let amt = fs :: copy (& from , & to) . with_context (| | { format ! ("failed to copy '{}' to '{}'" , from . as_ref () . display () , to . as_ref () . display ()) }) ? ; Ok (amt) } }
};
}
