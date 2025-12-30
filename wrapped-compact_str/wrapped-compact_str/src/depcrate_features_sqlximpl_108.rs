// Generated macro for impl_108 (impl)
macro_rules! Depcrate_features_sqlximpl_108 {
() => {
// Module: crate::features::sqlx
// Provides: {"impl_108"}
// Dependencies: {}
# [cfg (feature = "sqlx-mysql")] # [cfg_attr (docsrs , doc (cfg (feature = "sqlx-mysql")))] impl < 'q > Encode < 'q , sqlx :: MySql > for CompactString { fn encode_by_ref (& self , buf : & mut < sqlx :: MySql as Database > :: ArgumentBuffer < 'q > ,) -> Result < IsNull , BoxDynError > { Encode :: < '_ , sqlx :: MySql > :: encode_by_ref (& self . as_str () , buf) } # [inline] fn produces (& self) -> Option < < sqlx :: MySql as Database > :: TypeInfo > { < & str as Encode < '_ , sqlx :: MySql > > :: produces (& self . as_str ()) } # [inline] fn size_hint (& self) -> usize { < & str as Encode < '_ , sqlx :: MySql > > :: size_hint (& self . as_str ()) } }
};
}
