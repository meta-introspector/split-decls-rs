// Generated macro for IncompleteInsertStatement (struct)
macro_rules! Depcrate_query_builder_insert_statementIncompleteInsertStatement {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"IncompleteInsertStatement"}
// Dependencies: {}
# [doc = " The structure returned by [`insert_into`]."] # [doc = ""] # [doc = " The provided methods [`values`] and [`default_values`] will insert"] # [doc = " data into the targeted table."] # [doc = ""] # [doc = " [`insert_into`]: crate::insert_into()"] # [doc = " [`values`]: IncompleteInsertStatement::values()"] # [doc = " [`default_values`]: IncompleteInsertStatement::default_values()"] # [derive (Debug , Clone , Copy)] # [must_use = "Queries are only executed when calling `load`, `get_result` or similar."] pub struct IncompleteInsertStatement < T , Op = Insert > { target : T , operator : Op , }
};
}
