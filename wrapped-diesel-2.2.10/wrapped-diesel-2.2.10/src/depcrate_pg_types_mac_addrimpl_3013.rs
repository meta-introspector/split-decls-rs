// Generated macro for impl_3013 (impl)
macro_rules! Depcrate_pg_types_mac_addrimpl_3013 {
() => {
// Module: crate::pg::types::mac_addr
// Provides: {"impl_3013"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < MacAddr , Pg > for [u8 ; 6] { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_all (& self [..]) . map (| _ | IsNull :: No) . map_err (Into :: into) } }
};
}
