// Generated macro for ExecuteDsl (trait)
macro_rules! Depcrate_query_dsl_load_dslExecuteDsl {
() => {
// Module: crate::query_dsl::load_dsl
// Provides: {"ExecuteDsl"}
// Dependencies: {}
# [doc = " The `execute` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`RunQueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `execute` from generic code."] # [doc = ""] # [doc = " [`RunQueryDsl`]: crate::RunQueryDsl"] pub trait ExecuteDsl < Conn : Connection < Backend = DB > , DB : Backend = < Conn as Connection > :: Backend > : Sized { # [doc = " Execute this command"] fn execute (query : Self , conn : & mut Conn) -> QueryResult < usize > ; }
};
}
