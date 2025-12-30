// Generated macro for impl_2827 (impl)
macro_rules! Depcrate_pg_types_arrayimpl_2827 {
() => {
// Module: crate::pg::types::array
// Provides: {"impl_2827"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < ST , T > ToSql < Array < ST > , Pg > for Vec < T > where ST : 'static , [T] : ToSql < Array < ST > , Pg > , T : fmt :: Debug , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { (self as & [T]) . to_sql (out) } }
};
}
