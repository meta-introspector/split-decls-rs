// Generated macro for impl_122 (impl)
macro_rules! Depcrate_canonicalimpl_122 {
() => {
// Module: crate::canonical
// Provides: {"impl_122"}
// Dependencies: {}
impl < A > EdgeSet < A > { fn empty () -> Self { Self { set : Vec :: new () } } fn map < F , B > (self , f : F) -> EdgeSet < B > where F : Fn (A) -> Option < B > , { let new_set = self . set . into_iter () . map (| edge | edge . map (& f)) . collect () ; EdgeSet { set : new_set } } }
};
}
