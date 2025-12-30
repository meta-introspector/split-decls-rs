// Generated macro for impl_29 (impl)
macro_rules! Depcrate_mapimpl_29 {
() => {
// Module: crate::map
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a , K , V > fmt :: Debug for Entries < 'a , K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
