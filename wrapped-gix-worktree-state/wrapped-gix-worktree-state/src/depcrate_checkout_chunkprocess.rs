// Generated macro for process (function)
macro_rules! Depcrate_checkout_chunkprocess {
() => {
// Module: crate::checkout::chunk
// Provides: {"process"}
// Dependencies: {}
pub fn process < 'entry , Find > (entries_with_paths : impl Iterator < Item = (& 'entry mut gix_index :: Entry , & 'entry BStr) > , files : & AtomicUsize , bytes : & AtomicUsize , delayed_filter_results : & mut Vec < DelayedFilteredStream < 'entry > > , ctx : & mut Context < Find > ,) -> Result < Outcome < 'entry > , checkout :: Error > where Find : gix_object :: Find + Clone , { let mut delayed_symlinks = Vec :: new () ; let mut collisions = Vec :: new () ; let mut errors = Vec :: new () ; let mut bytes_written = 0 ; let mut files_in_chunk = 0 ; for (entry , entry_path) in entries_with_paths { if entry . flags . contains (gix_index :: entry :: Flags :: SKIP_WORKTREE) { files . fetch_add (1 , Ordering :: Relaxed) ; files_in_chunk += 1 ; continue ; } if entry . mode == gix_index :: entry :: Mode :: SYMLINK { delayed_symlinks . push ((entry , entry_path)) ; continue ; } match checkout_entry_handle_result (entry , entry_path , & mut errors , & mut collisions , files , bytes , ctx) ? { entry :: Outcome :: Written { bytes } => { bytes_written += bytes as u64 ; files_in_chunk += 1 ; } entry :: Outcome :: Delayed (delayed) => delayed_filter_results . push (delayed) , } } Ok (Outcome { bytes_written , files : files_in_chunk , errors , collisions , delayed_symlinks , delayed_paths_unknown : Vec :: new () , delayed_paths_unprocessed : Vec :: new () , }) }
};
}
