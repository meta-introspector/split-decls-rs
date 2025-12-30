// Generated macro for Join (struct)
macro_rules! Depcrate_query_source_joinsJoin {
() => {
// Module: crate::query_source::joins
// Provides: {"Join"}
// Dependencies: {}
# [doc = " A query source representing the join between two tables"] pub struct Join < Left : QuerySource , Right : QuerySource , Kind > { left : FromClause < Left > , right : FromClause < Right > , kind : Kind , }
};
}
