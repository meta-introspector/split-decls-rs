// Generated macro for impl_3108 (impl)
macro_rules! Depcrate_pg_types_rangesimpl_3108 {
() => {
// Module: crate::pg::types::ranges
// Provides: {"impl_3108"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < ST , T > ToSql < Nullable < Range < ST > > , Pg > for (Bound < T > , Bound < T >) where ST : 'static , (Bound < T > , Bound < T >) : ToSql < Range < ST > , Pg > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { ToSql :: < Range < ST > , Pg > :: to_sql (self , out) } }
};
}
