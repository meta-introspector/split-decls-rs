// Generated macro for impl_2826 (impl)
macro_rules! Depcrate_pg_types_arrayimpl_2826 {
() => {
// Module: crate::pg::types::array
// Provides: {"impl_2826"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < ST , T > ToSql < Nullable < Array < ST > > , Pg > for [T] where [T] : ToSql < Array < ST > , Pg > , ST : 'static , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { ToSql :: < Array < ST > , Pg > :: to_sql (self , out) } }
};
}
