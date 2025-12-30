// Generated macro for Concat (struct)
macro_rules! Depcrate_expression_operatorsConcat {
() => {
// Module: crate::expression::operators
// Provides: {"Concat"}
// Dependencies: {}
# [doc = " This type represents a string concat operator"] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , public_fields (left , right))] # [derive (Debug , Clone , Copy , QueryId , DieselNumericOps , ValidGrouping)] pub struct Concat < L , R > { # [doc = " The left side expression of the operator"] pub (crate) left : L , # [doc = " The right side expression of the operator"] pub (crate) right : R , }
};
}
