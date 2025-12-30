// Generated macro for Div (trait)
macro_rules! Depcrate_sql_types_opsDiv {
() => {
// Module: crate::sql_types::ops
// Provides: {"Div"}
// Dependencies: {}
# [doc = " Represents SQL types which can be divided."] pub trait Div { # [doc = " The SQL type which this one can be divided by"] type Rhs : SqlType ; # [doc = " The SQL type of the result of dividing `Self` by `Rhs`"] type Output : SqlType ; }
};
}
