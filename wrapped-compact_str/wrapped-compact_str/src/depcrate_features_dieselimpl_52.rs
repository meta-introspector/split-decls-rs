// Generated macro for impl_52 (impl)
macro_rules! Depcrate_features_dieselimpl_52 {
() => {
// Module: crate::features::diesel
// Provides: {"impl_52"}
// Dependencies: {}
impl < DB > serialize :: ToSql < sql_types :: Text , DB > for CompactString where DB : backend :: Backend , str : serialize :: ToSql < sql_types :: Text , DB > , { fn to_sql < 'b > (& 'b self , out : & mut serialize :: Output < 'b , '_ , DB >) -> serialize :: Result { self . as_str () . to_sql (out) } }
};
}
