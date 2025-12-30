// Generated macro for MigrationName (trait)
macro_rules! Depcrate_migrationMigrationName {
() => {
// Module: crate::migration
// Provides: {"MigrationName"}
// Dependencies: {}
# [doc = " Represents the name of a migration"] # [doc = ""] # [doc = " Users should threat this as `impl Display` type,"] # [doc = " for implementors of custom migration types"] # [doc = " this opens the possibility to roll out their own versioning"] # [doc = " schema."] pub trait MigrationName : Display { # [doc = " The version corresponding to the current migration name"] fn version (& self) -> MigrationVersion < '_ > ; }
};
}
