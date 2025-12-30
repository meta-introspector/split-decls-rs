// Generated macro for impl_3947 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_chronoimpl_3947 {
() => {
// Module: crate::sqlite::types::date_and_time::chrono
// Provides: {"impl_3947"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "chrono"))] impl ToSql < Date , Sqlite > for NaiveDate { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { out . set_value (self . format (DATE_FORMAT) . to_string ()) ; Ok (IsNull :: No) } }
};
}
