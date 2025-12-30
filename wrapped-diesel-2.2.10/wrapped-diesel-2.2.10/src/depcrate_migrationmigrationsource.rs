// Generated macro for MigrationSource (trait)
macro_rules! Depcrate_migrationMigrationSource {
() => {
// Module: crate::migration
// Provides: {"MigrationSource"}
// Dependencies: {}
# [doc = " A migration source is an entity that can be used"] # [doc = " to receive a number of migrations from."] pub trait MigrationSource < DB : Backend > { # [doc = " Get a list of migrations associated with this"] # [doc = " migration source."] fn migrations (& self) -> Result < Vec < Box < dyn Migration < DB > > > > ; }
};
}
