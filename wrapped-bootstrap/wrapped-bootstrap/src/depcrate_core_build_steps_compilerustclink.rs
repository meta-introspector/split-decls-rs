// Generated macro for RustcLink (struct)
macro_rules! Depcrate_core_build_steps_compileRustcLink {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"RustcLink"}
// Dependencies: {}
# [doc = " `RustcLink` copies compiler rlibs from a rustc build into a compiler sysroot."] # [doc = " It works with (potentially up to) three compilers:"] # [doc = " - `build_compiler` is a compiler that built rustc rlibs"] # [doc = " - `sysroot_compiler` is a compiler into whose sysroot we will copy the rlibs"] # [doc = "   - In most situations, `build_compiler` == `sysroot_compiler`"] # [doc = " - `target_compiler` is the compiler whose rlibs were built. It is not represented explicitly"] # [doc = "   in this step, rather we just read the rlibs from a rustc build stamp of `build_compiler`."] # [doc = ""] # [doc = " This is necessary for tools using `rustc_private`, where the previous compiler will build"] # [doc = " a tool against the next compiler."] # [doc = " To build a tool against a compiler, the rlibs of that compiler that it links against"] # [doc = " must be in the sysroot of the compiler that's doing the compiling."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] struct RustcLink { # [doc = " This compiler **built** some rustc, whose rlibs we will copy into a sysroot."] build_compiler : Compiler , # [doc = " This is the compiler into whose sysroot we want to copy the built rlibs."] # [doc = " In most cases, it will correspond to `build_compiler`."] sysroot_compiler : Compiler , target : TargetSelection , # [doc = " Not actually used; only present to make sure the cache invalidation is correct."] crates : Vec < String > , }
};
}
