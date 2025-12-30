// Generated macro for macro_154 (macro)
macro_rules! Depcrate_arbitrary__core_itermacro_154 {
() => {
// Module: crate::arbitrary::_core::iter
// Provides: {"macro_154"}
// Dependencies: {}
arbitrary ! (['a , T : 'a + Clone , A : Arbitrary + Iterator < Item = &'a T >] Cloned < A >, SMapped < A , Self >, A :: Parameters ; args => static_map (any_with ::< A > (args) , Iterator :: cloned)) ;
};
}
