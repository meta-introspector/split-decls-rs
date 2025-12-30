// Generated macro for impl_2828 (impl)
macro_rules! Depcrate_pg_types_arrayimpl_2828 {
() => {
// Module: crate::pg::types::array
// Provides: {"impl_2828"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < ST , T > ToSql < Nullable < Array < ST > > , Pg > for Vec < T > where ST : 'static , Vec < T > : ToSql < Array < ST > , Pg > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { ToSql :: < Array < ST > , Pg > :: to_sql (self , out) } }
};
}
