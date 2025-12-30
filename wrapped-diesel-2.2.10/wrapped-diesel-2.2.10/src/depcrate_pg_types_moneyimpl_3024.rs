// Generated macro for impl_3024 (impl)
macro_rules! Depcrate_pg_types_moneyimpl_3024 {
() => {
// Module: crate::pg::types::money
// Provides: {"impl_3024"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < Money , Pg > for PgMoney { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { ToSql :: < BigInt , Pg > :: to_sql (& self . 0 , out) } }
};
}
