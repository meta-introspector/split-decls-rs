// Generated macro for impl_212 (impl)
macro_rules! Depcrate_linear_mapimpl_212 {
() => {
// Module: crate::linear_map
// Provides: {"impl_212"}
// Dependencies: {}
impl < K , V , S : LinearMapStorage < K , V > + ? Sized > fmt :: Debug for LinearMapInner < K , V , S > where K : Eq + fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } }
};
}
