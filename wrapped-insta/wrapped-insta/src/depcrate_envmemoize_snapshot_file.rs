// Generated macro for memoize_snapshot_file (function)
macro_rules! Depcrate_envmemoize_snapshot_file {
() => {
// Module: crate::env
// Provides: {"memoize_snapshot_file"}
// Dependencies: {}
# [doc = " Memoizes a snapshot file in the reference file, as part of removing unreferenced snapshots."] pub fn memoize_snapshot_file (snapshot_file : & Path) { if let Ok (path) = env :: var ("INSTA_SNAPSHOT_REFERENCES_FILE") { let mut f = fs :: OpenOptions :: new () . append (true) . create (true) . open (path) . unwrap () ; f . write_all (format ! ("{}\n" , snapshot_file . display ()) . as_bytes ()) . unwrap () ; } }
};
}
