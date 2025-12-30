// Generated macro for impl_242 (impl)
macro_rules! Depcrate_tagimpl_242 {
() => {
// Module: crate::tag
// Provides: {"impl_242"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , T > FixedTag for Cow < 'a , T > where T : ToOwned + ? Sized , & 'a T : FixedTag , { const TAG : Tag = < & 'a T > :: TAG ; }
};
}
