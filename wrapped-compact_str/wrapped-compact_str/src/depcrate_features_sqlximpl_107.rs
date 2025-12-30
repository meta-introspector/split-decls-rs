// Generated macro for impl_107 (impl)
macro_rules! Depcrate_features_sqlximpl_107 {
() => {
// Module: crate::features::sqlx
// Provides: {"impl_107"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "sqlx")))] impl < 'r , DB > Decode < 'r , DB > for CompactString where DB : Database , for < 'x > & 'x str : Decode < 'x , DB > + Type < DB > , { fn decode (value : < DB as Database > :: ValueRef < 'r >) -> Result < Self , BoxDynError > { let value = < & str as Decode < DB > > :: decode (value) ? ; Ok (value . try_to_compact_string () ?) } }
};
}
