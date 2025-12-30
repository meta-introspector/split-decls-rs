// Generated macro for Outcome (struct)
macro_rules! Depcrate_extension_decodeOutcome {
() => {
// Module: crate::extension::decode
// Provides: {"Outcome"}
// Dependencies: {}
# [derive (Default)] pub (crate) struct Outcome { pub tree : Option < extension :: Tree > , pub link : Option < extension :: Link > , pub resolve_undo : Option < extension :: resolve_undo :: Paths > , pub untracked : Option < extension :: UntrackedCache > , pub fs_monitor : Option < extension :: FsMonitor > , pub is_sparse : bool , pub offset_table : bool , pub end_of_index : bool , }
};
}
