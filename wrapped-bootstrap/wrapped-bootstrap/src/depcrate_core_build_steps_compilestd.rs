// Generated macro for Std (struct)
macro_rules! Depcrate_core_build_steps_compileStd {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"Std"}
// Dependencies: {}
# [doc = " Build a standard library for the given `target` using the given `build_compiler`."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Std { pub target : TargetSelection , # [doc = " Compiler that builds the standard library."] pub build_compiler : Compiler , # [doc = " Whether to build only a subset of crates in the standard library."] # [doc = ""] # [doc = " This shouldn't be used from other steps; see the comment on [`Rustc`]."] crates : Vec < String > , # [doc = " When using download-rustc, we need to use a new build of `std` for running unit tests of Std itself,"] # [doc = " but we need to use the downloaded copy of std for linking to rustdoc. Allow this to be overridden by `builder.ensure` from other steps."] force_recompile : bool , extra_rust_args : & 'static [& 'static str] , is_for_mir_opt_tests : bool , }
};
}
