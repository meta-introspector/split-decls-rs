// Generated macro for BuiltRustc (struct)
macro_rules! Depcrate_core_build_steps_compileBuiltRustc {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"BuiltRustc"}
// Dependencies: {}
# [doc = " Represents information about a built rustc."] # [derive (Clone , Debug)] pub struct BuiltRustc { # [doc = " The compiler that actually built this *rustc*."] # [doc = " This can be different from the *build_compiler* passed to the `Rustc` step because of"] # [doc = " uplifting."] pub build_compiler : Compiler , }
};
}
