// Generated macro for impl_36 (impl)
macro_rules! Depcrate_mapimpl_36 {
() => {
// Module: crate::map
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'a , K , V > fmt :: Debug for Keys < 'a , K , V > where K : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
