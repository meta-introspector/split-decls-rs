// Generated macro for CompilerForCheck (struct)
macro_rules! Depcrate_core_build_steps_checkCompilerForCheck {
() => {
// Module: crate::core::build_steps::check
// Provides: {"CompilerForCheck"}
// Dependencies: {}
# [doc = " Represents a compiler that can check something."] # [doc = ""] # [doc = " If the compiler was created for `Mode::ToolRustcPrivate` or `Mode::Codegen`, it will also contain"] # [doc = " .rmeta artifacts from rustc that was already checked using `build_compiler`."] # [doc = ""] # [doc = " All steps that use this struct in a \"general way\" (i.e. they don't know exactly what kind of"] # [doc = " thing is being built) should call `configure_cargo` to ensure that the rmeta artifacts are"] # [doc = " properly linked, if present."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct CompilerForCheck { build_compiler : Compiler , rustc_rmeta_sysroot : Option < RmetaSysroot > , std_rmeta_sysroot : Option < RmetaSysroot > , }
};
}
