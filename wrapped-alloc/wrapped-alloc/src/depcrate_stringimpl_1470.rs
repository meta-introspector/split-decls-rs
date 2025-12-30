// Generated macro for impl_1470 (impl)
macro_rules! Depcrate_stringimpl_1470 {
() => {
// Module: crate::string
// Provides: {"impl_1470"}
// Dependencies: {}
# [doc = " # Panics"] # [doc = ""] # [doc = " In this implementation, the `to_string` method panics"] # [doc = " if the `Display` implementation returns an error."] # [doc = " This indicates an incorrect `Display` implementation"] # [doc = " since `fmt::Write for String` never returns an error itself."] # [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : fmt :: Display + ? Sized > ToString for T { # [inline] fn to_string (& self) -> String { < Self as SpecToString > :: spec_to_string (self) } }
};
}
