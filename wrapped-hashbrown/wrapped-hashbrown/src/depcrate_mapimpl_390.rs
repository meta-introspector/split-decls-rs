// Generated macro for impl_390 (impl)
macro_rules! Depcrate_mapimpl_390 {
() => {
// Module: crate::map
// Provides: {"impl_390"}
// Dependencies: {}
impl < K , V : Debug > fmt :: Debug for ValuesMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter () . map (| (_ , val) | val)) . finish () } }
};
}
