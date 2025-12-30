// Generated macro for find_snapshot_roots (function)
macro_rules! Depcrate_cargofind_snapshot_roots {
() => {
// Module: crate::cargo
// Provides: {"find_snapshot_roots"}
// Dependencies: {}
# [doc = " Find snapshot roots within a package"] pub (crate) fn find_snapshot_roots (package : & Package) -> Vec < PathBuf > { let mut roots = std :: collections :: HashSet :: new () ; if let Some (manifest) = package . manifest_path . parent () { roots . insert (manifest . as_std_path () . to_path_buf ()) ; } for target in & package . targets { if target . kind . iter () . any (| kind | kind == "custom-build") { continue ; } let root = target . src_path . parent () . unwrap () . as_std_path () ; roots . insert (root . to_path_buf ()) ; } let roots : Vec < _ > = roots . into_iter () . collect () ; let canonical_roots : Vec < _ > = roots . iter () . filter_map (| x | x . canonicalize () . ok ()) . sorted_by_key (| x | x . as_os_str () . len ()) . collect () ; canonical_roots . clone () . into_iter () . filter (| root | { ! canonical_roots . iter () . any (| x | root . starts_with (x) && root != x) }) . collect () }
};
}
