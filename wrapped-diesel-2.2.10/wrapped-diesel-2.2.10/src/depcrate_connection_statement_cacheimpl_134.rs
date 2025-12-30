// Generated macro for impl_134 (impl)
macro_rules! Depcrate_connection_statement_cacheimpl_134 {
() => {
// Module: crate::connection::statement_cache
// Provides: {"impl_134"}
// Dependencies: {}
impl < DB > StatementCacheKey < DB > where DB : Backend , DB :: QueryBuilder : Default , DB :: TypeMetadata : Clone , { # [doc = " Create a new statement cache key for the given query source"] # [allow (unreachable_pub)] pub fn for_source (maybe_type_id : Option < TypeId > , source : & dyn QueryFragmentForCachedStatement < DB > , bind_types : & [DB :: TypeMetadata] , backend : & DB ,) -> QueryResult < Self > { match maybe_type_id { Some (id) => Ok (StatementCacheKey :: Type (id)) , None => { let sql = source . construct_sql (backend) ? ; Ok (StatementCacheKey :: Sql { sql , bind_types : bind_types . into () , }) } } } # [doc = " Get the sql for a given query source based"] # [doc = ""] # [doc = " This is an optimization that may skip constructing the query string"] # [doc = " twice if it's already part of the current cache key"] # [allow (unreachable_pub)] pub fn sql (& self , source : & dyn QueryFragmentForCachedStatement < DB > , backend : & DB ,) -> QueryResult < Cow < '_ , str > > { match * self { StatementCacheKey :: Type (_) => source . construct_sql (backend) . map (Cow :: Owned) , StatementCacheKey :: Sql { ref sql , .. } => Ok (Cow :: Borrowed (sql)) , } } }
};
}
