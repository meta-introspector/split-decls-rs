// Generated macro for tests (module)
macro_rules! Depcrate_compaction_filter_factorytests {
() => {
// Module: crate::compaction_filter_factory
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: compaction_filter :: Decision ; use crate :: { Options , DB } ; use std :: ffi :: CString ; struct CountFilter (u16 , CString) ; impl CompactionFilter for CountFilter { fn filter (& mut self , _level : u32 , _key : & [u8] , _value : & [u8]) -> crate :: CompactionDecision { self . 0 += 1 ; if self . 0 > 2 { Decision :: Remove } else { Decision :: Keep } } fn name (& self) -> & CStr { & self . 1 } } struct TestFactory (CString) ; impl CompactionFilterFactory for TestFactory { type Filter = CountFilter ; fn create (& mut self , _context : CompactionFilterContext) -> Self :: Filter { CountFilter (0 , CString :: new ("CountFilter") . unwrap ()) } fn name (& self) -> & CStr { & self . 0 } } # [test] fn compaction_filter_factory_test () { let tempdir = tempfile :: Builder :: new () . prefix ("_rust_rocksdb_filter_factory_test") . tempdir () . expect ("Failed to create temporary path for the _rust_rocksdb_filter_factory_test.") ; let path = tempdir . path () ; let mut opts = Options :: default () ; opts . create_if_missing (true) ; opts . set_compaction_filter_factory (TestFactory (CString :: new ("TestFactory") . unwrap ())) ; { let db = DB :: open (& opts , path) . unwrap () ; let _r = db . put (b"k1" , b"a") ; let _r = db . put (b"_rk" , b"b") ; let _r = db . put (b"%k" , b"c") ; db . compact_range (None :: < & [u8] > , None :: < & [u8] >) ; assert_eq ! (db . get (b"%k1") . unwrap () , None) ; } let result = DB :: destroy (& opts , path) ; assert ! (result . is_ok ()) ; } }
};
}
