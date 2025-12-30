// Generated macro for CopyTarget (trait)
macro_rules! Depcrate_pg_query_builder_copyCopyTarget {
() => {
// Module: crate::pg::query_builder::copy
// Provides: {"CopyTarget"}
// Dependencies: {}
# [doc = " A expression that could be used as target/source for `COPY FROM` and `COPY TO` commands"] # [doc = ""] # [doc = " This trait is implemented for any table type and for tuples of columns from the same table"] pub trait CopyTarget { # [doc = " The table targeted by the command"] type Table : Table ; # [doc = " The sql side type of the target expression"] type SqlType : SqlType ; # [doc (hidden)] fn walk_target (pass : crate :: query_builder :: AstPass < '_ , '_ , Pg >) -> crate :: QueryResult < () > ; }
};
}
