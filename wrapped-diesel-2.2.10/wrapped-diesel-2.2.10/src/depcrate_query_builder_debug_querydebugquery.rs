// Generated macro for DebugQuery (struct)
macro_rules! Depcrate_query_builder_debug_queryDebugQuery {
() => {
// Module: crate::query_builder::debug_query
// Provides: {"DebugQuery"}
// Dependencies: {}
# [doc = " A struct that implements `fmt::Display` and `fmt::Debug` to show the SQL"] # [doc = " representation of a query."] # [doc = ""] # [doc = " The `Display` implementation will be the exact query sent to the server,"] # [doc = " plus a comment with the values of the bind parameters. The `Debug`"] # [doc = " implementation is more structured, and able to be pretty printed."] # [doc = ""] # [doc = " See [`debug_query`] for usage examples."] # [doc = ""] # [doc = " [`debug_query`]: crate::query_builder::debug_query()"] pub struct DebugQuery < 'a , T : 'a , DB > { pub (crate) query : & 'a T , _marker : PhantomData < DB > , }
};
}
