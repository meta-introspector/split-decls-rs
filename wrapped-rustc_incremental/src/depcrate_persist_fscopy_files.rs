// Generated macro for copy_files (function)
macro_rules! Depcrate_persist_fscopy_files {
() => {
// Module: crate::persist::fs
// Provides: {"copy_files"}
// Dependencies: {}
fn copy_files (sess : & Session , target_dir : & Path , source_dir : & Path) -> Result < bool , () > { let lock_file_path = lock_file_path (source_dir) ; let Ok (_lock) = flock :: Lock :: new (& lock_file_path , false , false , false ,) else { return Err (()) ; } ; let Ok (source_dir_iterator) = source_dir . read_dir () else { return Err (()) ; } ; let mut files_linked = 0 ; let mut files_copied = 0 ; for entry in source_dir_iterator { match entry { Ok (entry) => { let file_name = entry . file_name () ; let target_file_path = target_dir . join (file_name) ; let source_path = entry . path () ; debug ! ("copying into session dir: {}" , source_path . display ()) ; match link_or_copy (source_path , target_file_path) { Ok (LinkOrCopy :: Link) => files_linked += 1 , Ok (LinkOrCopy :: Copy) => files_copied += 1 , Err (_) => return Err (()) , } } Err (_) => return Err (()) , } } if sess . opts . unstable_opts . incremental_info { eprintln ! ("[incremental] session directory: \
                  {files_linked} files hard-linked") ; eprintln ! ("[incremental] session directory: \
                 {files_copied} files copied") ; } Ok (files_linked > 0 || files_copied == 0) }
};
}
