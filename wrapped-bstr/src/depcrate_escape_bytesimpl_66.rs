// Generated macro for impl_66 (impl)
macro_rules! Depcrate_escape_bytesimpl_66 {
() => {
// Module: crate::escape_bytes
// Provides: {"impl_66"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < I : Iterator < Item = char > > UnescapeBytes < I > { pub (crate) fn new < T : IntoIterator < IntoIter = I > > (t : T ,) -> UnescapeBytes < I > { UnescapeBytes { it : t . into_iter () , state : UnescapeState :: Start } } }
};
}
