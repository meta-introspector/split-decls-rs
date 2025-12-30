// Generated macro for impl_2547 (impl)
macro_rules! Depcrate_mysql_typesimpl_2547 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2547"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl ToSql < Bool , Mysql > for bool { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { let int_value = i32 :: from (* self) ; < i32 as ToSql < Integer , Mysql > > :: to_sql (& int_value , & mut out . reborrow ()) } }
};
}
