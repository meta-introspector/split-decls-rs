// Generated macro for Many (struct)
macro_rules! Depcrate_expression_array_comparisonMany {
() => {
// Module: crate::expression::array_comparison
// Provides: {"Many"}
// Dependencies: {}
# [doc = " Query dsl node for an `IN (values)` clause containing"] # [doc = " a variable number of bind values."] # [doc = ""] # [doc = " Third party backend can customize the [`QueryFragment`]"] # [doc = " implementation of this query dsl node via"] # [doc = " [`SqlDialect::ArrayComparison`]. The default"] # [doc = " implementation does generate one bind per value"] # [doc = " in the `values` field."] # [doc = ""] # [doc = " Diesel provides an optimized implementation for Postgresql"] # [doc = " like database systems that bind all values with one"] # [doc = " bind value of the type `Array<ST>` instead."] # [derive (Debug , Clone)] pub struct Many < ST , I > { # [doc = " The values contained in the `IN (values)` clause"] pub values : Vec < I > , p : PhantomData < ST > , }
};
}
