// Generated macro for impl_216 (impl)
macro_rules! Depcrate_tree_builderimpl_216 {
() => {
// Module: crate::tree_builder
// Provides: {"impl_216"}
// Dependencies: {}
impl < 'a , Handle : 'a > ActiveFormattingView < 'a , Handle > { fn iter (& 'a self) -> impl Iterator < Item = (usize , & 'a Handle , & 'a Tag) > + 'a { ActiveFormattingIter { iter : self . data . iter () . enumerate () . rev () , } } }
};
}
