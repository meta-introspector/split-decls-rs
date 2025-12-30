// Generated macro for InsertStatement (struct)
macro_rules! Depcrate_query_builder_insert_statementInsertStatement {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"InsertStatement"}
// Dependencies: {}
# [doc = " A fully constructed insert statement."] # [doc = ""] # [doc = " The parameters of this struct represent:"] # [doc = ""] # [doc = " - `T`: The table we are inserting into"] # [doc = " - `U`: The data being inserted"] # [doc = " - `Op`: The operation being performed. The specific types used to represent"] # [doc = "   this are private, but correspond to SQL such as `INSERT` or `REPLACE`."] # [doc = "   You can safely rely on the default type representing `INSERT`"] # [doc = " - `Ret`: The `RETURNING` clause of the query. The specific types used to"] # [doc = "   represent this are private. You can safely rely on the default type"] # [doc = "   representing a query without a `RETURNING` clause."] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , public_fields (operator , target , records , returning))] # [derive (Debug , Copy , Clone)] # [must_use = "Queries are only executed when calling `load`, `get_result` or similar."] pub struct InsertStatement < T : QuerySource , U , Op = Insert , Ret = NoReturningClause > { # [doc = " The operator used by this InsertStatement"] # [doc = ""] # [doc = " Corresponds to either `Insert` or `Replace`"] operator : Op , # [doc = " The table we are inserting into"] target : T , # [doc = " The data which should be inserted"] records : U , # [doc = " An optional returning clause"] returning : Ret , into_clause : T :: FromClause , }
};
}
