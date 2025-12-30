// Generated macro for compaction_filter_test (function)
macro_rules! Depcrate_compaction_filtercompaction_filter_test {
() => {
// Module: crate::compaction_filter
// Provides: {"compaction_filter_test"}
// Dependencies: {}
# [test] fn compaction_filter_test () { use crate :: { Options , DB } ; let tempdir = tempfile :: Builder :: new () . prefix ("_rust_rocksdb_filter_test") . tempdir () . expect ("Failed to create temporary path for the _rust_rocksdb_filter_test") ; let path = tempdir . path () ; let mut opts = Options :: default () ; opts . create_if_missing (true) ; opts . set_compaction_filter ("test" , test_filter) ; { let db = DB :: open (& opts , path) . unwrap () ; let _r = db . put (b"k1" , b"a") ; let _r = db . put (b"_k" , b"b") ; let _r = db . put (b"%k" , b"c") ; db . compact_range (None :: < & [u8] > , None :: < & [u8] >) ; assert_eq ! (&* db . get (b"k1") . unwrap () . unwrap () , b"a") ; assert ! (db . get (b"_k") . unwrap () . is_none ()) ; assert_eq ! (&* db . get (b"%k") . unwrap () . unwrap () , b"secret") ; } let result = DB :: destroy (& opts , path) ; assert ! (result . is_ok ()) ; }
};
}
