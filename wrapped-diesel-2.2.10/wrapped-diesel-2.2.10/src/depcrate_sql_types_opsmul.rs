// Generated macro for Mul (trait)
macro_rules! Depcrate_sql_types_opsMul {
() => {
// Module: crate::sql_types::ops
// Provides: {"Mul"}
// Dependencies: {}
# [doc = " Represents SQL types which can be multiplied."] pub trait Mul { # [doc = " The SQL type which this can be multiplied by"] type Rhs : SqlType ; # [doc = " The SQL type of the result of multiplying `Self` by `Rhs`"] type Output : SqlType ; }
};
}
