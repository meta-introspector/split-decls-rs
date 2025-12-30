// Generated macro for copy_and_manifest (function)
macro_rules! Depcrate_generatorcopy_and_manifest {
() => {
// Module: crate::generator
// Provides: {"copy_and_manifest"}
// Dependencies: {}
# [doc = " Copies the `src` directory recursively to `dst`, writing `manifest.in` too."] fn copy_and_manifest (src : & Path , dst : & Path , bulk_dirs : & str) -> Result < () > { let mut manifest = create_new_file (dst . join ("manifest.in")) ? ; let bulk_dirs : Vec < _ > = bulk_dirs . split (',') . filter (| s | ! s . is_empty ()) . map (Path :: new) . collect () ; let mut paths = BTreeSet :: new () ; copy_with_callback (src , dst , | path , file_type | { if path . components () . filter_map (| c | c . as_os_str () . to_str ()) . any (| s | s . contains ('\\')) { bail ! ("rust-installer doesn't support '\\' in path components: {:?}" , path) ; } let normalized_string ; let mut string = path . to_str () . ok_or_else (| | { format_err ! ("rust-installer doesn't support non-Unicode paths: {:?}" , path) }) ? ; if string . contains ('\\') { normalized_string = string . replace ('\\' , "/") ; string = & normalized_string ; } if file_type . is_dir () { if bulk_dirs . contains (& path) { paths . insert (format ! ("dir:{}\n" , string)) ; } } else { if ! bulk_dirs . iter () . any (| d | path . starts_with (d)) { paths . insert (format ! ("file:{}\n" , string)) ; } } Ok (()) }) ? ; for path in paths { manifest . write_all (path . as_bytes ()) ? ; } Ok (()) }
};
}
