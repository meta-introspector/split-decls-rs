// Generated macro for impl_106 (impl)
macro_rules! Depcrate_features_sqlximpl_106 {
() => {
// Module: crate::features::sqlx
// Provides: {"impl_106"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "sqlx")))] impl < DB > Type < DB > for CompactString where DB : Database , for < 'x > & 'x str : Type < DB > , { # [inline] fn type_info () -> < DB as Database > :: TypeInfo { < & str as Type < DB > > :: type_info () } }
};
}
