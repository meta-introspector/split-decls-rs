// Generated macro for impl_166 (impl)
macro_rules! Depcrate_core_build_steps_compileimpl_166 {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"impl_166"}
// Dependencies: {}
impl RustcLink { # [doc = " Copy rlibs from the build compiler that build this `rustc` into the sysroot of that"] # [doc = " build compiler."] fn from_rustc (rustc : Rustc) -> Self { Self { build_compiler : rustc . build_compiler , sysroot_compiler : rustc . build_compiler , target : rustc . target , crates : rustc . crates , } } # [doc = " Copy rlibs **built** by `build_compiler` into the sysroot of `sysroot_compiler`."] fn from_build_compiler_and_sysroot (build_compiler : Compiler , sysroot_compiler : Compiler , target : TargetSelection , crates : Vec < String > ,) -> Self { Self { build_compiler , sysroot_compiler , target , crates } } }
};
}
