// Generated macro for dsl (module)
macro_rules! Depcrate_pg_expressiondsl {
() => {
// Module: crate::pg::expression
// Provides: {"dsl"}
// Dependencies: {}
# [doc = " PostgreSQL specific expression DSL methods."] # [doc = ""] # [doc = " This module will be glob imported by"] # [doc = " [`diesel::dsl`](crate::dsl) when compiled with the `feature ="] # [doc = " \"postgres\"` flag."] pub mod dsl { # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] # [doc (inline)] # [allow (deprecated)] pub use super :: array_comparison :: { all , any } ; # [doc (inline)] pub use super :: array :: array ; # [doc (inline)] pub use super :: extensions :: * ; # [doc (inline)] pub use super :: functions :: * ; }
};
}
