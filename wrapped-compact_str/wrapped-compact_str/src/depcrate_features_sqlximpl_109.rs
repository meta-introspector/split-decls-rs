// Generated macro for impl_109 (impl)
macro_rules! Depcrate_features_sqlximpl_109 {
() => {
// Module: crate::features::sqlx
// Provides: {"impl_109"}
// Dependencies: {}
# [cfg (feature = "sqlx-postgres")] # [cfg_attr (docsrs , doc (cfg (feature = "sqlx-postgres")))] impl < 'q > Encode < 'q , sqlx :: Postgres > for CompactString { fn encode_by_ref (& self , buf : & mut < sqlx :: Postgres as Database > :: ArgumentBuffer < 'q > ,) -> Result < IsNull , BoxDynError > { Encode :: < '_ , sqlx :: Postgres > :: encode_by_ref (& self . as_str () , buf) } # [inline] fn produces (& self) -> Option < < sqlx :: Postgres as Database > :: TypeInfo > { < & str as Encode < '_ , sqlx :: Postgres > > :: produces (& self . as_str ()) } # [inline] fn size_hint (& self) -> usize { < & str as Encode < '_ , sqlx :: Postgres > > :: size_hint (& self . as_str ()) } }
};
}
