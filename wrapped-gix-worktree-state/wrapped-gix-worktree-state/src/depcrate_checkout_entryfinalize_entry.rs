// Generated macro for finalize_entry (function)
macro_rules! Depcrate_checkout_entryfinalize_entry {
() => {
// Module: crate::checkout::entry
// Provides: {"finalize_entry"}
// Dependencies: {}
# [doc = " Close `file` and store its stats in `entry`, possibly setting `file` executable."] pub (crate) fn finalize_entry (entry : & mut gix_index :: Entry , file : std :: fs :: File , # [cfg_attr (windows , allow (unused_variables))] set_executable_after_creation : bool ,) -> Result < () , crate :: checkout :: Error > { # [cfg (unix)] if set_executable_after_creation { set_executable (& file) ? ; } entry . stat = Stat :: from_fs (& gix_index :: fs :: Metadata :: from_file (& file) ?) ? ; file . close () ? ; Ok (()) }
};
}
