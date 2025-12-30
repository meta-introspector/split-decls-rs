// Generated macro for Outcome (struct)
macro_rules! Depcrate_checkout_chunkOutcome {
() => {
// Module: crate::checkout::chunk
// Provides: {"Outcome"}
// Dependencies: {}
# [derive (Default)] pub struct Outcome < 'a > { pub collisions : Vec < checkout :: Collision > , pub errors : Vec < checkout :: ErrorRecord > , pub delayed_symlinks : Vec < (& 'a mut gix_index :: Entry , & 'a BStr) > , pub bytes_written : u64 , pub files : usize , # [doc = " Relative paths that the process listed as 'delayed' even though we never passed them."] pub delayed_paths_unknown : Vec < BString > , # [doc = " All paths that were left unprocessed, because they were never listed by the process even though we passed them."] pub delayed_paths_unprocessed : Vec < BString > , }
};
}
