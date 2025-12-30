// Generated macro for append_tar_entry (function)
macro_rules! Depcrate_writeappend_tar_entry {
() => {
// Module: crate::write
// Provides: {"append_tar_entry"}
// Dependencies: {}
# [cfg (any (feature = "tar" , feature = "tar_gz"))] fn append_tar_entry < W : std :: io :: Write > (ar : & mut tar :: Builder < W > , buf : & mut Vec < u8 > , mut entry : gix_worktree_stream :: Entry < '_ > , mtime_seconds_since_epoch : i64 , opts : & Options ,) -> Result < () , Error > { let mut header = tar :: Header :: new_gnu () ; header . set_mtime (mtime_seconds_since_epoch as u64) ; header . set_entry_type (tar_entry_type (entry . mode)) ; header . set_mode (if entry . mode . is_executable () { 0o755 } else { 0o644 }) ; buf . clear () ; std :: io :: copy (& mut entry , buf) ? ; let path = gix_path :: from_bstr (add_prefix (entry . relative_path () , opts . tree_prefix . as_ref ())) ; header . set_size (buf . len () as u64) ; if entry . mode . is_link () { use bstr :: ByteSlice ; let target = gix_path :: from_bstr (buf . as_bstr ()) ; header . set_entry_type (tar :: EntryType :: Symlink) ; header . set_size (0) ; ar . append_link (& mut header , path , target) ? ; } else { ar . append_data (& mut header , path , buf . as_slice ()) ? ; } Ok (()) }
};
}
