// Generated macro for SqlFnOutput (trait)
macro_rules! Depcrate_functionsSqlFnOutput {
() => {
// Module: crate::functions
// Provides: {"SqlFnOutput"}
// Dependencies: {}
# [doc = " Result of an SQL function"] pub trait SqlFnOutput { # [doc = " Converts Rust value to SQLite value with an optional subtype"] fn to_sql (& self) -> Result < (ToSqlOutput < '_ > , SubType) > ; }
};
}
