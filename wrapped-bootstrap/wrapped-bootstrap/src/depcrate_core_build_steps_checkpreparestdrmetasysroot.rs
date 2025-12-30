// Generated macro for PrepareStdRmetaSysroot (struct)
macro_rules! Depcrate_core_build_steps_checkPrepareStdRmetaSysroot {
() => {
// Module: crate::core::build_steps::check
// Provides: {"PrepareStdRmetaSysroot"}
// Dependencies: {}
# [doc = " Checks std using the given `build_compiler` for the given `target`, and produces"] # [doc = " a sysroot in the build directory that stores the generated .rmeta files."] # [doc = ""] # [doc = " This step exists so that we can store the generated .rmeta artifacts into a separate"] # [doc = " directory, instead of copying them into the sysroot of `build_compiler`, which would"] # [doc = " \"pollute\" it (that is especially problematic for the external stage0 rustc)."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] struct PrepareStdRmetaSysroot { build_compiler : Compiler , target : TargetSelection , }
};
}
