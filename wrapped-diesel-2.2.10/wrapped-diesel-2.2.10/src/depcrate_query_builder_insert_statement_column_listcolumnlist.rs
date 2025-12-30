// Generated macro for ColumnList (trait)
macro_rules! Depcrate_query_builder_insert_statement_column_listColumnList {
() => {
// Module: crate::query_builder::insert_statement::column_list
// Provides: {"ColumnList"}
// Dependencies: {}
# [doc = " Represents the column list for use in an insert statement."] # [doc = ""] # [doc = " This trait is implemented by columns and tuples of columns."] pub trait ColumnList { # [doc = " The table these columns belong to"] type Table ; # [doc = " Generate the SQL for this column list."] # [doc = ""] # [doc = " Column names must *not* be qualified."] fn walk_ast < DB : Backend > (& self , out : AstPass < '_ , '_ , DB >) -> QueryResult < () > ; }
};
}
