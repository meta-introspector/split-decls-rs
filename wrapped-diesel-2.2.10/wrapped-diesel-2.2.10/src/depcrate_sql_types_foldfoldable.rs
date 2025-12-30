// Generated macro for Foldable (trait)
macro_rules! Depcrate_sql_types_foldFoldable {
() => {
// Module: crate::sql_types::fold
// Provides: {"Foldable"}
// Dependencies: {}
# [doc = " Represents SQL types which can be used with `SUM` and `AVG`"] pub trait Foldable : SingleValue { # [doc = " The SQL type of `sum(this_type)`"] type Sum : SqlType + SingleValue ; # [doc = " The SQL type of `avg(this_type)`"] type Avg : SqlType + SingleValue ; }
};
}
