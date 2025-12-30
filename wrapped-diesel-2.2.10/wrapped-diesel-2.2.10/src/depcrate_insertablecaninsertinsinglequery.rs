// Generated macro for CanInsertInSingleQuery (trait)
macro_rules! Depcrate_insertableCanInsertInSingleQuery {
() => {
// Module: crate::insertable
// Provides: {"CanInsertInSingleQuery"}
// Dependencies: {}
pub trait CanInsertInSingleQuery < DB : Backend > { # [doc = " How many rows will this query insert?"] # [doc = ""] # [doc = " This function should only return `None` when the query is valid on all"] # [doc = " backends, regardless of how many rows get inserted."] fn rows_to_insert (& self) -> Option < usize > ; }
};
}
