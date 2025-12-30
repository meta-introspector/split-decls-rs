// Generated macro for option_type (function)
macro_rules! Depcrate_generatoroption_type {
() => {
// Module: crate::generator
// Provides: {"option_type"}
// Dependencies: {}
fn option_type () -> TokenStream { # [cfg (feature = "std")] quote ! { :: std :: option :: Option } # [cfg (not (feature = "std"))] quote ! { :: core :: option :: Option } }
};
}
