// Generated macro for impl_739 (impl)
macro_rules! Depcrate_test_dbimpl_739 {
() => {
// Module: crate::test_db
// Provides: {"impl_739"}
// Dependencies: {}
impl TestDB { pub (crate) fn log (& self , f : impl FnOnce ()) -> Vec < salsa :: Event > { * self . events . lock () . unwrap () = Some (Vec :: new ()) ; f () ; self . events . lock () . unwrap () . take () . unwrap () } pub (crate) fn log_executed (& self , f : impl FnOnce ()) -> (Vec < String > , Vec < salsa :: Event >) { let events = self . log (f) ; let executed = events . iter () . filter_map (| e | match e . kind { salsa :: EventKind :: WillExecute { database_key } => { let ingredient = (self as & dyn salsa :: Database) . ingredient_debug_name (database_key . ingredient_index ()) ; Some (ingredient . to_string ()) } _ => None , }) . collect () ; (executed , events) } }
};
}
