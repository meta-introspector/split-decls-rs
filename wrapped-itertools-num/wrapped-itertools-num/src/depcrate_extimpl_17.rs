// Generated macro for impl_17 (impl)
macro_rules! Depcrate_extimpl_17 {
() => {
// Module: crate::ext
// Provides: {"impl_17"}
// Dependencies: {}
impl < I , S > ExactSizeIterator for Cumsum < I , S > where I : Iterator , S : Add < I :: Item , Output = S > , S : Zero + Clone , { }
};
}
