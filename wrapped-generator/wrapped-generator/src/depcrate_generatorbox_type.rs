// Generated macro for box_type (function)
macro_rules! Depcrate_generatorbox_type {
() => {
// Module: crate::generator
// Provides: {"box_type"}
// Dependencies: {}
fn box_type () -> TokenStream { # [cfg (feature = "std")] quote ! { :: std :: boxed :: Box } # [cfg (not (feature = "std"))] quote ! { :: alloc :: boxed :: Box } }
};
}
