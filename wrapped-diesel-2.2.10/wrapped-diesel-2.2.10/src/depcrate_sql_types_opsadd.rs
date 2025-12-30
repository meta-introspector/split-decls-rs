// Generated macro for Add (trait)
macro_rules! Depcrate_sql_types_opsAdd {
() => {
// Module: crate::sql_types::ops
// Provides: {"Add"}
// Dependencies: {}
# [doc = " Represents SQL types which can be added."] pub trait Add { # [doc = " The SQL type which can be added to this one"] type Rhs : SqlType ; # [doc = " The SQL type of the result of adding `Rhs` to `Self`"] type Output : SqlType ; }
};
}
