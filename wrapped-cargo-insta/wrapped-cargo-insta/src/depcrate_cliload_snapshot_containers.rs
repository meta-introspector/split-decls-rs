// Generated macro for load_snapshot_containers (function)
macro_rules! Depcrate_cliload_snapshot_containers {
() => {
// Module: crate::cli
// Provides: {"load_snapshot_containers"}
// Dependencies: {}
# [allow (clippy :: type_complexity)] fn load_snapshot_containers < 'a > (loc : & 'a LocationInfo ,) -> Result < (Vec < (SnapshotContainer , & 'a Package) > , HashSet < PathBuf >) , Box < dyn Error > > { let mut roots = HashSet :: new () ; let mut snapshot_containers = vec ! [] ; debug_assert ! (! loc . packages . is_empty ()) ; for package in & loc . packages { for root in find_snapshot_roots (package) { roots . insert (root . clone ()) ; for snapshot_container in find_pending_snapshots (& root , & loc . exts , loc . find_flags) { snapshot_containers . push ((snapshot_container ? , package)) ; } } } snapshot_containers . sort_by (| a , b | a . 0 . snapshot_sort_key () . cmp (& b . 0 . snapshot_sort_key ())) ; Ok ((snapshot_containers , roots)) }
};
}
