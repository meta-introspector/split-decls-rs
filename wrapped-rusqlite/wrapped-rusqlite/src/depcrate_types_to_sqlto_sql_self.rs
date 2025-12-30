// Generated macro for to_sql_self (macro)
macro_rules! Depcrate_types_to_sqlto_sql_self {
() => {
// Module: crate::types::to_sql
// Provides: {"to_sql_self"}
// Dependencies: {}
macro_rules ! to_sql_self (($ t : ty) => (impl ToSql for $ t { # [inline] fn to_sql (& self) -> Result < ToSqlOutput <'_ >> { Ok (ToSqlOutput :: from (* self)) } })) ;
};
}
