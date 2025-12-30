// Generated macro for QueryBuilder (trait)
macro_rules! Depcrate_query_builderQueryBuilder {
() => {
// Module: crate::query_builder
// Provides: {"QueryBuilder"}
// Dependencies: {}
# [doc = " Constructs a SQL query from a Diesel AST."] # [doc = ""] # [doc = " The only reason you should ever need to interact with this trait is if you"] # [doc = " are extending Diesel with support for a new backend. Plugins which extend"] # [doc = " the query builder with new capabilities will interact with [`AstPass`]"] # [doc = " instead."] # [doc = ""] pub trait QueryBuilder < DB : Backend > { # [doc = " Add `sql` to the end of the query being constructed."] fn push_sql (& mut self , sql : & str) ; # [doc = " Quote `identifier`, and add it to the end of the query being"] # [doc = " constructed."] fn push_identifier (& mut self , identifier : & str) -> QueryResult < () > ; # [doc = " Add a placeholder for a bind parameter to the end of the query being"] # [doc = " constructed."] fn push_bind_param (& mut self) ; # [doc = " Increases the internal counter for bind parameters without adding the"] # [doc = " bind parameter itself to the query"] fn push_bind_param_value_only (& mut self) { } # [doc = " Returns the constructed SQL query."] fn finish (self) -> String ; }
};
}
