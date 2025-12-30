// Generated macro for is_mutex_guard (function)
macro_rules! Depcrate_await_holding_invalidis_mutex_guard {
() => {
// Module: crate::await_holding_invalid
// Provides: {"is_mutex_guard"}
// Dependencies: {}
fn is_mutex_guard (cx : & LateContext < '_ > , def_id : DefId) -> bool { match cx . tcx . get_diagnostic_name (def_id) { Some (name) => matches ! (name , sym :: MutexGuard | sym :: RwLockReadGuard | sym :: RwLockWriteGuard) , None => paths :: PARKING_LOT_GUARDS . iter () . any (| guard | guard . matches (cx , def_id)) , } }
};
}
