// Generated macro for impl_79 (impl)
macro_rules! Depcrate_ordered_mapimpl_79 {
() => {
// Module: crate::ordered_map
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a , K , V > fmt :: Debug for Values < 'a , K , V > where V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
