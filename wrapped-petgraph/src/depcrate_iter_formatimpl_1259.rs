// Generated macro for impl_1259 (impl)
macro_rules! Depcrate_iter_formatimpl_1259 {
() => {
// Module: crate::iter_format
// Provides: {"impl_1259"}
// Dependencies: {}
impl < F , I , K , V > fmt :: Debug for DebugMap < F > where F : Fn () -> I , I : IntoIterator < Item = (K , V) > , K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_map () . entries ((self . 0) ()) . finish () } }
};
}
