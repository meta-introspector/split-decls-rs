// Generated macro for make_snapshot_walker (function)
macro_rules! Depcrate_walkmake_snapshot_walker {
() => {
// Module: crate::walk
// Provides: {"make_snapshot_walker"}
// Dependencies: {}
# [doc = " Creates a walker for snapshots & pending snapshots within a package. The"] # [doc = " walker returns snapshots ending in any of the supplied extensions, any of"] # [doc = " the supplied extensions with a `.new` suffix, and `.pending-snap` files."] pub (crate) fn make_snapshot_walker (package_root : & Path , extensions : & [& str] , flags : FindFlags ,) -> Walk { let mut builder = WalkBuilder :: new (package_root) ; builder . standard_filters (! flags . include_ignored) ; if flags . include_hidden { builder . hidden (false) ; } else { builder . filter_entry (| e | e . file_type () . map_or (false , | x | x . is_file ()) || ! is_hidden (e)) ; } let mut override_builder = OverrideBuilder :: new (package_root) ; extensions . iter () . flat_map (| ext | [format ! ("*.{ext}") , format ! ("*.{ext}.new")]) . chain (std :: iter :: once ("*.pending-snap" . to_string ())) . for_each (| pattern | { override_builder . add (& pattern) . unwrap () ; }) ; builder . overrides (override_builder . build () . unwrap ()) ; let root_path = package_root . to_path_buf () ; builder . filter_entry (move | entry | { if entry . file_type () . map_or (false , | ft | ft . is_dir ()) && entry . path () . join ("Cargo.toml") . exists () && entry . path () != root_path { return false ; } if entry . path () . file_name () == Some (OsStr :: new ("target")) { return false ; } true }) ; builder . build () }
};
}
