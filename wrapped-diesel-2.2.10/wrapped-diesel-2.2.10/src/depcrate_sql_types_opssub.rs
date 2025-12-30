// Generated macro for Sub (trait)
macro_rules! Depcrate_sql_types_opsSub {
() => {
// Module: crate::sql_types::ops
// Provides: {"Sub"}
// Dependencies: {}
# [doc = " Represents SQL types which can be subtracted."] pub trait Sub { # [doc = " The SQL type which can be subtracted from this one"] type Rhs : SqlType ; # [doc = " The SQL type of the result of subtracting `Rhs` from `Self`"] type Output : SqlType ; }
};
}
