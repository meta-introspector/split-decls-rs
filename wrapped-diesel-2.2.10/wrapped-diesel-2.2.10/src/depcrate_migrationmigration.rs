// Generated macro for Migration (trait)
macro_rules! Depcrate_migrationMigration {
() => {
// Module: crate::migration
// Provides: {"Migration"}
// Dependencies: {}
# [doc = " Represents a migration that interacts with diesel"] pub trait Migration < DB : Backend > { # [doc = " Apply this migration"] fn run (& self , conn : & mut dyn BoxableConnection < DB >) -> Result < () > ; # [doc = " Revert this migration"] fn revert (& self , conn : & mut dyn BoxableConnection < DB >) -> Result < () > ; # [doc = " Get a the attached metadata for this migration"] fn metadata (& self) -> & dyn MigrationMetadata ; # [doc = " Get the name of the current migration"] # [doc = ""] # [doc = " The provided name is used by migration harness"] # [doc = " to get the version of a migration and to"] # [doc = " as something to that is displayed and allows"] # [doc = " user to identify a specific migration"] fn name (& self) -> & dyn MigrationName ; }
};
}
