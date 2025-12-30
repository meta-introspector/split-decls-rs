// Generated macro for impl_407 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_407 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_407"}
// Dependencies: {}
# [stable (feature = "map_values_mut" , since = "1.10.0")] impl < K , V : fmt :: Debug > fmt :: Debug for ValuesMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter () . map (| (_ , val) | val)) . finish () } }
};
}
