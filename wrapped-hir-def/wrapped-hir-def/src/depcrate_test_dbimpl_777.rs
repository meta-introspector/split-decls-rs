// Generated macro for impl_777 (impl)
macro_rules! Depcrate_test_dbimpl_777 {
() => {
// Module: crate::test_db
// Provides: {"impl_777"}
// Dependencies: {}
impl Clone for TestDB { fn clone (& self) -> Self { Self { storage : self . storage . clone () , files : self . files . clone () , crates_map : self . crates_map . clone () , events : self . events . clone () , nonce : Nonce :: new () , } } }
};
}
