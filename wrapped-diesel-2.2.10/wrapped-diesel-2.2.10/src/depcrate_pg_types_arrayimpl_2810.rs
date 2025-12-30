// Generated macro for impl_2810 (impl)
macro_rules! Depcrate_pg_types_arrayimpl_2810 {
() => {
// Module: crate::pg::types::array
// Provides: {"impl_2810"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < T > HasSqlType < Array < T > > for Pg where Pg : HasSqlType < T > , { fn metadata (lookup : & mut Self :: MetadataLookup) -> PgTypeMetadata { match < Pg as HasSqlType < T > > :: metadata (lookup) . 0 { Ok (tpe) => PgTypeMetadata :: new (tpe . array_oid , 0) , c @ Err (_) => PgTypeMetadata (c) , } } }
};
}
