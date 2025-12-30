// Generated macro for PgMetadataLookup (trait)
macro_rules! Depcrate_pg_metadata_lookupPgMetadataLookup {
() => {
// Module: crate::pg::metadata_lookup
// Provides: {"PgMetadataLookup"}
// Dependencies: {}
# [doc = " Determines the OID of types at runtime"] # [doc = ""] # [doc = " Custom implementations of `Connection<Backend = Pg>` should not implement this trait directly."] # [doc = " Instead `GetPgMetadataCache` should be implemented, afterwards the generic implementation will provide"] # [doc = " the necessary functions to perform the type lookup."] # [cfg (feature = "postgres_backend")] pub trait PgMetadataLookup { # [doc = " Determine the type metadata for the given `type_name`"] # [doc = ""] # [doc = " This function should only be used for user defined types, or types which"] # [doc = " come from an extension. This function may perform a SQL query to look"] # [doc = " up the type. For built-in types, a static OID should be preferred."] fn lookup_type (& mut self , type_name : & str , schema : Option < & str >) -> PgTypeMetadata ; # [doc = " Convert this lookup instance to a `std::any::Any` pointer"] # [doc = ""] # [doc = " Implementing this method is required to support `#[derive(MultiConnection)]`"] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] fn as_any < 'a > (& mut self) -> & mut (dyn std :: any :: Any + 'a) where Self : 'a , { unimplemented ! () } }
};
}
