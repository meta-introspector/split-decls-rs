// Generated macro for result_type (function)
macro_rules! Depcrate_generatorresult_type {
() => {
// Module: crate::generator
// Provides: {"result_type"}
// Dependencies: {}
fn result_type () -> TokenStream { # [cfg (feature = "std")] quote ! { :: std :: result :: Result } # [cfg (not (feature = "std"))] quote ! { :: core :: result :: Result } }
};
}
