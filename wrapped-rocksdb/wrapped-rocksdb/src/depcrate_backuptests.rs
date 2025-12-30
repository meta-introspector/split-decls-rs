// Generated macro for tests (module)
macro_rules! Depcrate_backuptests {
() => {
// Module: crate::backup
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: BackupEngineOptions ; # [test] fn test_sync () { let dir = tempfile :: Builder :: new () . prefix ("rocksdb-test-sync") . tempdir () . expect ("Failed to create temporary path for db.") ; let mut opts = BackupEngineOptions :: new (dir . path ()) . unwrap () ; assert ! (opts . get_sync ()) ; opts . set_sync (false) ; assert ! (! opts . get_sync ()) ; } }
};
}
