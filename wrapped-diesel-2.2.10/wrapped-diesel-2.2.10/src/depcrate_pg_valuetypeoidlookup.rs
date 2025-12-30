// Generated macro for TypeOidLookup (trait)
macro_rules! Depcrate_pg_valueTypeOidLookup {
() => {
// Module: crate::pg::value
// Provides: {"TypeOidLookup"}
// Dependencies: {}
# [doc = " This is a helper trait to defer a type oid"] # [doc = " lookup to a later point in time"] # [doc = ""] # [doc = " This is mainly used in the `PgConnection`"] # [doc = " implementation so that we do not need to call"] # [doc = " into libpq if we do not need the type oid."] # [doc = ""] # [doc = " Backend implementations based on pure rustc"] # [doc = " database connection crates can likely reuse"] # [doc = " the implementation for `NonZeroU32` here instead"] # [doc = " of providing their own custom implementation"] # [cfg_attr (docsrs , doc (cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")))] # [allow (unreachable_pub)] pub trait TypeOidLookup { # [doc = " Lookup the type oid for the current value"] fn lookup (& self) -> NonZeroU32 ; }
};
}
