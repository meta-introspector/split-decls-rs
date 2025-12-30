// Generated macro for MigrationVersion (struct)
macro_rules! Depcrate_migrationMigrationVersion {
() => {
// Module: crate::migration
// Provides: {"MigrationVersion"}
// Dependencies: {}
# [doc = " A migration version identifier"] # [doc = ""] # [doc = " This is used by the migration harness to place migrations"] # [doc = " in order, therefore two different instances of this type"] # [doc = " must be sortable"] # [derive (Debug , Hash , PartialEq , Eq , PartialOrd , Ord , FromSqlRow , AsExpression)] # [diesel (sql_type = Text)] pub struct MigrationVersion < 'a > (Cow < 'a , str >) ;
};
}
