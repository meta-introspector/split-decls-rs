// Generated macro for impl_111 (impl)
macro_rules! Depcrate_features_sqlximpl_111 {
() => {
// Module: crate::features::sqlx
// Provides: {"impl_111"}
// Dependencies: {}
# [cfg (feature = "sqlx-sqlite")] # [cfg_attr (docsrs , doc (cfg (feature = "sqlx-sqlite")))] impl < 'q > Encode < 'q , sqlx :: Sqlite > for CompactString { fn encode (self , buf : & mut < sqlx :: Sqlite as Database > :: ArgumentBuffer < 'q > ,) -> Result < IsNull , BoxDynError > { Encode :: < '_ , sqlx :: Sqlite > :: encode (self . into_string () , buf) } fn encode_by_ref (& self , buf : & mut < sqlx :: Sqlite as Database > :: ArgumentBuffer < 'q > ,) -> Result < IsNull , BoxDynError > { Encode :: < '_ , sqlx :: Sqlite > :: encode (alloc :: string :: String :: from (self . as_str ()) , buf) } # [inline] fn produces (& self) -> Option < < sqlx :: Sqlite as Database > :: TypeInfo > { < & str as Encode < '_ , sqlx :: Sqlite > > :: produces (& self . as_str ()) } # [inline] fn size_hint (& self) -> usize { < & str as Encode < '_ , sqlx :: Sqlite > > :: size_hint (& self . as_str ()) } }
};
}
