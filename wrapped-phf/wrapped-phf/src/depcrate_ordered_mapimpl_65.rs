// Generated macro for impl_65 (impl)
macro_rules! Depcrate_ordered_mapimpl_65 {
() => {
// Module: crate::ordered_map
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'a , K , V > fmt :: Debug for Entries < 'a , K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
