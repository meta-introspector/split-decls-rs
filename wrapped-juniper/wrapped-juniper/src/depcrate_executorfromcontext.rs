// Generated macro for FromContext (trait)
macro_rules! Depcrate_executorFromContext {
() => {
// Module: crate::executor
// Provides: {"FromContext"}
// Dependencies: {}
# [doc = " Conversion trait for context types"] # [doc = ""] # [doc = " Used to support different context types for different parts of an"] # [doc = " application. By making each `GraphQL` type only aware of as much"] # [doc = " context as it needs to, isolation and robustness can be"] # [doc = " improved. Implement this trait if you have contexts that can"] # [doc = " generally be converted between each other."] # [doc = ""] # [doc = " The empty tuple `()` can be converted into from any context type,"] # [doc = " making it suitable for `GraphQL` that don't need _any_ context to"] # [doc = " work, e.g. scalars or enums."] pub trait FromContext < T > { # [doc = " Perform the conversion"] fn from (value : & T) -> & Self ; }
};
}
