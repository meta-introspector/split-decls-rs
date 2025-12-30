// Generated macro for impl_519 (impl)
macro_rules! Depcrate_displayimpl_519 {
() => {
// Module: crate::display
// Provides: {"impl_519"}
// Dependencies: {}
impl DisplayTarget { pub fn from_crate (db : & dyn HirDatabase , krate : Crate) -> Self { let edition = krate . data (db) . edition ; Self { krate , edition } } }
};
}
