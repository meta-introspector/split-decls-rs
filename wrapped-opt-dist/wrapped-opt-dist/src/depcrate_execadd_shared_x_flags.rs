// Generated macro for add_shared_x_flags (function)
macro_rules! Depcrate_execadd_shared_x_flags {
() => {
// Module: crate::exec
// Provides: {"add_shared_x_flags"}
// Dependencies: {}
fn add_shared_x_flags (env : & Environment , cmd : CmdBuilder) -> CmdBuilder { if env . is_fast_try_build () { cmd . arg ("--set") . arg ("rust.llvm-bitcode-linker=false") . arg ("--set") . arg ("build.extended=false") . arg ("--set") . arg ("rust.codegen-backends=['llvm']") . arg ("--set") . arg ("rust.deny-warnings=false") } else { cmd } }
};
}
