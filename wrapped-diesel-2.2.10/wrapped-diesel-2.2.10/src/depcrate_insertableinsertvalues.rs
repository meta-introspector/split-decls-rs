// Generated macro for InsertValues (trait)
macro_rules! Depcrate_insertableInsertValues {
() => {
// Module: crate::insertable
// Provides: {"InsertValues"}
// Dependencies: {}
pub trait InsertValues < DB : Backend , T : Table > : QueryFragment < DB > { fn column_names (& self , out : AstPass < '_ , '_ , DB >) -> QueryResult < () > ; }
};
}
