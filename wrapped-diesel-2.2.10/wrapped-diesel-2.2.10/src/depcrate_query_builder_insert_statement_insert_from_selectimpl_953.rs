// Generated macro for impl_953 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_from_selectimpl_953 {
() => {
// Module: crate::query_builder::insert_statement::insert_from_select
// Provides: {"impl_953"}
// Dependencies: {}
impl < Select , Columns > InsertFromSelect < Select , Columns > { # [doc = " Construct a new `InsertFromSelect` where the target column list is"] # [doc = " `T::AllColumns`."] pub fn new < T > (query : Select) -> Self where T : Table < AllColumns = Columns > , Columns : SelectableExpression < T > + NonAggregate , { Self { query , columns : T :: all_columns () , } } # [doc = " Replace the target column list"] pub fn with_columns < C > (self , columns : C) -> InsertFromSelect < Select , C > { InsertFromSelect { query : self . query , columns , } } }
};
}
