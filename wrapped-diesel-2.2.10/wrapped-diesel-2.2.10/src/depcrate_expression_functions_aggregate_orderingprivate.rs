// Generated macro for private (module)
macro_rules! Depcrate_expression_functions_aggregate_orderingprivate {
() => {
// Module: crate::expression::functions::aggregate_ordering
// Provides: {"private"}
// Dependencies: {}
mod private { use crate :: sql_types :: { IntoNullable , SingleValue , SqlOrd , SqlType } ; pub trait SqlOrdAggregate : SingleValue { type Ret : SqlType + SingleValue ; } impl < T > SqlOrdAggregate for T where T : SqlOrd + IntoNullable + SingleValue , T :: Nullable : SqlType + SingleValue , { type Ret = T :: Nullable ; } }
};
}
