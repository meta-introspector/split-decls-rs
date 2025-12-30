// Generated macro for impl_776 (impl)
macro_rules! Depcrate_test_dbimpl_776 {
() => {
// Module: crate::test_db
// Provides: {"impl_776"}
// Dependencies: {}
impl Default for TestDB { fn default () -> Self { let events = < Arc < Mutex < Option < Vec < salsa :: Event > > > > > :: default () ; let mut this = Self { storage : salsa :: Storage :: new (Some (Box :: new ({ let events = events . clone () ; move | event | { let mut events = events . lock () . unwrap () ; if let Some (events) = & mut * events { events . push (event) ; } } }))) , events , files : Default :: default () , crates_map : Default :: default () , nonce : Nonce :: new () , } ; this . set_expand_proc_attr_macros_with_durability (true , Durability :: HIGH) ; this . set_all_crates (Arc :: new (Box :: new ([]))) ; CrateGraphBuilder :: default () . set_in_db (& mut this) ; this } }
};
}
