// Generated macro for process_paths_for_mod_files (function)
macro_rules! Depcrate_module_styleprocess_paths_for_mod_files {
() => {
// Module: crate::module_style
// Provides: {"process_paths_for_mod_files"}
// Dependencies: {}
# [doc = " For each `path` we add each folder component to `folder_segments` and if the file name"] # [doc = " is `mod.rs` we add it's parent folder to `mod_folders`."] fn process_paths_for_mod_files < 'a > (path : & 'a Path , folder_segments : & mut FxIndexSet < & 'a OsStr > , mod_folders : & mut FxHashSet < & 'a OsStr > ,) { let mut comp = path . components () . rev () . peekable () ; let _ : Option < _ > = comp . next () ; if path . ends_with ("mod.rs") { mod_folders . insert (comp . peek () . map (| c | c . as_os_str ()) . unwrap_or_default ()) ; } let folders = comp . filter_map (| c | if let Component :: Normal (s) = c { Some (s) } else { None }) ; folder_segments . extend (folders) ; }
};
}
