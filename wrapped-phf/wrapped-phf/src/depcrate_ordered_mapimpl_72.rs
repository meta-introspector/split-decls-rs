// Generated macro for impl_72 (impl)
macro_rules! Depcrate_ordered_mapimpl_72 {
() => {
// Module: crate::ordered_map
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a , K , V > fmt :: Debug for Keys < 'a , K , V > where K : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
