// Generated macro for expression_types (module)
macro_rules! Depcrate_expressionexpression_types {
() => {
// Module: crate::expression
// Provides: {"expression_types"}
// Dependencies: {}
# [doc = " Possible types for []`Expression::SqlType`]"] # [doc = ""] pub mod expression_types { use super :: { QueryMetadata , TypedExpressionType } ; use crate :: backend :: Backend ; use crate :: sql_types :: SingleValue ; # [doc = " Query nodes with this expression type do not have a statically at compile"] # [doc = " time known expression type."] # [doc = ""] # [doc = " An example for such a query node in diesel itself, is `sql_query` as"] # [doc = " we do not know which fields are returned from such a query at compile time."] # [doc = ""] # [doc = " For loading values from queries returning a type of this expression, consider"] # [doc = " using [`#[derive(QueryableByName)]`](derive@crate::deserialize::QueryableByName)"] # [doc = " on the corresponding result type."] # [doc = ""] # [derive (Clone , Copy , Debug)] pub struct Untyped ; # [doc = " Query nodes witch cannot be part of a select clause."] # [doc = ""] # [doc = " If you see an error message containing `FromSqlRow` and this type"] # [doc = " recheck that you have written a valid select clause"] # [derive (Debug , Clone , Copy)] pub struct NotSelectable ; impl TypedExpressionType for Untyped { } impl TypedExpressionType for NotSelectable { } impl < ST > TypedExpressionType for ST where ST : SingleValue { } impl < DB : Backend > QueryMetadata < Untyped > for DB { fn row_metadata (_ : & mut DB :: MetadataLookup , row : & mut Vec < Option < DB :: TypeMetadata > >) { row . push (None) } } }
};
}
