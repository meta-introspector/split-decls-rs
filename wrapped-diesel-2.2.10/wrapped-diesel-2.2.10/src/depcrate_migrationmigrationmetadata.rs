// Generated macro for MigrationMetadata (trait)
macro_rules! Depcrate_migrationMigrationMetadata {
() => {
// Module: crate::migration
// Provides: {"MigrationMetadata"}
// Dependencies: {}
# [doc = " This trait is designed to customize the behaviour"] # [doc = " of the default migration harness of diesel"] # [doc = ""] # [doc = " Any new customization option will be added"] # [doc = " as new function here. Each new function"] # [doc = " will have a default implementation"] # [doc = " returning the old a value corresponding"] # [doc = " to the old uncustomized behaviour"] pub trait MigrationMetadata { # [doc = " Whether the current migration is executed in a transaction or not"] # [doc = ""] # [doc = " By default each migration is executed in a own transaction, but"] # [doc = " certain operations (like creating an index on an existing column)"] # [doc = " requires running the migration without transaction."] # [doc = ""] # [doc = " By default this function returns true"] fn run_in_transaction (& self) -> bool { true } }
};
}
