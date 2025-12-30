// Generated macro for TestDB (struct)
macro_rules! Depcrate_test_dbTestDB {
() => {
// Module: crate::test_db
// Provides: {"TestDB"}
// Dependencies: {}
# [salsa_macros :: db] pub (crate) struct TestDB { storage : salsa :: Storage < Self > , files : Arc < base_db :: Files > , crates_map : Arc < CratesMap > , events : Arc < Mutex < Option < Vec < salsa :: Event > > > > , nonce : Nonce , }
};
}
