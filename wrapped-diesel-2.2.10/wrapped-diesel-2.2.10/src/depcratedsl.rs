// Generated macro for dsl (module)
macro_rules! Depcratedsl {
() => {
// Module: crate
// Provides: {"dsl"}
// Dependencies: {}
# [allow (unknown_lints , ambiguous_glob_reexports)] pub mod dsl { # ! [doc = " Includes various helper types and bare functions which are named too"] # ! [doc = " generically to be included in prelude, but are often used when using Diesel."] # [doc (inline)] pub use crate :: helper_types :: * ; # [doc (inline)] pub use crate :: expression :: dsl :: * ; # [doc (inline)] pub use crate :: query_builder :: functions :: { delete , insert_into , insert_or_ignore_into , replace_into , select , sql_query , update , } ; # [doc (inline)] # [cfg (feature = "postgres_backend")] pub use crate :: query_builder :: functions :: { copy_from , copy_to } ; # [doc (inline)] pub use diesel_derives :: auto_type ; # [cfg (feature = "postgres_backend")] # [doc (inline)] pub use crate :: pg :: expression :: extensions :: OnlyDsl ; # [cfg (feature = "postgres_backend")] # [doc (inline)] pub use crate :: pg :: expression :: extensions :: TablesampleDsl ; }
};
}
