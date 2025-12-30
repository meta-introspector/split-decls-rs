// Generated macro for find_path_suggestion (function)
macro_rules! Depcrate_source_utilfind_path_suggestion {
() => {
// Module: crate::source_util
// Provides: {"find_path_suggestion"}
// Dependencies: {}
fn find_path_suggestion (source_map : & SourceMap , base_dir : & Path , wanted_path : & Path ,) -> Option < PathBuf > { let mut base_c = base_dir . components () ; let mut wanted_c = wanted_path . components () ; let mut without_base = None ; while let Some (wanted_next) = wanted_c . next () { if wanted_c . as_path () . file_name () . is_none () { break ; } while let Some (base_next) = base_c . next () { if base_next == wanted_next { without_base = Some (wanted_c . as_path ()) ; break ; } } } let root_absolute = without_base . into_iter () . map (PathBuf :: from) ; let base_dir_components = base_dir . components () . count () ; let max_parent_components = if base_dir . is_relative () { base_dir_components + 1 } else { base_dir_components . saturating_sub (1) } ; let mut prefix = PathBuf :: new () ; let add = std :: iter :: from_fn (| | { prefix . push ("..") ; Some (prefix . join (wanted_path)) }) . take (max_parent_components . min (3)) ; let mut trimmed_path = wanted_path ; let remove = std :: iter :: from_fn (| | { let mut components = trimmed_path . components () ; let removed = components . next () ? ; trimmed_path = components . as_path () ; let _ = trimmed_path . file_name () ? ; Some ([Some (trimmed_path . to_path_buf ()) , (removed != std :: path :: Component :: ParentDir) . then (| | Path :: new ("..") . join (trimmed_path)) ,]) }) . flatten () . flatten () . take (4) ; root_absolute . chain (add) . chain (remove) . find (| new_path | source_map . file_exists (& base_dir . join (& new_path))) }
};
}
