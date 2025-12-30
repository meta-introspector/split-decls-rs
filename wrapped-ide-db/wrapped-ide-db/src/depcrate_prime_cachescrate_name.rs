// Generated macro for crate_name (function)
macro_rules! Depcrate_prime_cachescrate_name {
() => {
// Module: crate::prime_caches
// Provides: {"crate_name"}
// Dependencies: {}
fn crate_name (db : & RootDatabase , krate : Crate) -> Symbol { krate . extra_data (db) . display_name . as_deref () . cloned () . unwrap_or_else (| | Symbol :: integer (salsa :: plumbing :: AsId :: as_id (& krate) . index () as usize)) }
};
}
