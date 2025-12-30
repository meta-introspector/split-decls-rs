// Generated macro for impl_594 (impl)
macro_rules! Depcrate_displayimpl_594 {
() => {
// Module: crate::display
// Provides: {"impl_594"}
// Dependencies: {}
impl DisplayTarget { pub fn from_crate (db : & dyn HirDatabase , krate : Crate) -> Self { let edition = krate . data (db) . edition ; Self { krate , edition } } }
};
}
