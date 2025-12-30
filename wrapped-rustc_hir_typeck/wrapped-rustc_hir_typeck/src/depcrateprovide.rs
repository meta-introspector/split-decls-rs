// Generated macro for provide (function)
macro_rules! Depcrateprovide {
() => {
// Module: crate
// Provides: {"provide"}
// Dependencies: {}
# [doc = " Adds query implementations to the [Providers] vtable, see [`rustc_middle::query`]"] pub fn provide (providers : & mut Providers) { * providers = Providers { method_autoderef_steps : method :: probe :: method_autoderef_steps , typeck , used_trait_imports , check_transmutes : intrinsicck :: check_transmutes , .. * providers } ; }
};
}
