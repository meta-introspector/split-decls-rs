// Generated macro for impl_2899 (impl)
macro_rules! Depcrate_pg_types_date_and_time_timeimpl_2899 {
() => {
// Module: crate::pg::types::date_and_time::time
// Provides: {"impl_2899"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "postgres_backend"))] impl ToSql < Timestamptz , Pg > for OffsetDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { let as_utc = self . to_offset (UtcOffset :: UTC) ; let primitive_date_time = PrimitiveDateTime :: new (as_utc . date () , as_utc . time ()) ; ToSql :: < Timestamptz , Pg > :: to_sql (& primitive_date_time , & mut out . reborrow ()) } }
};
}
