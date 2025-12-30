// Generated macro for assists (function)
macro_rules! Depcrate_testsassists {
() => {
// Module: crate::tests
// Provides: {"assists"}
// Dependencies: {}
fn assists (db : & RootDatabase , config : & AssistConfig , resolve : AssistResolveStrategy , range : ide_db :: FileRange ,) -> Vec < Assist > { hir :: attach_db (db , | | { HirDatabase :: zalsa_register_downcaster (db) ; crate :: assists (db , config , resolve , range) }) }
};
}
