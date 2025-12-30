// Generated macro for ValuesClause (struct)
macro_rules! Depcrate_query_builder_insert_statementValuesClause {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"ValuesClause"}
// Dependencies: {}
# [doc = " This type represents a values clause used as part of insert statements"] # [doc = ""] # [doc = " Diesel exposes this type for third party backends so that"] # [doc = " they can implement batch insert support"] # [cfg_attr (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] # [derive (Debug , Clone , Copy , QueryId)] pub struct ValuesClause < T , Tab > { # [doc = " Values to insert"] pub values : T , _marker : PhantomData < Tab > , }
};
}
