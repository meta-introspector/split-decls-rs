// Generated macro for impl_3949 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_chronoimpl_3949 {
() => {
// Module: crate::sqlite::types::date_and_time::chrono
// Provides: {"impl_3949"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "chrono"))] impl ToSql < Time , Sqlite > for NaiveTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { out . set_value (self . format (ENCODE_TIME_FORMAT) . to_string ()) ; Ok (IsNull :: No) } }
};
}
