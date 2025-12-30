// Generated macro for impl_181 (impl)
macro_rules! Depcrate_external_deps_llvmimpl_181 {
() => {
// Module: crate::external_deps::llvm
// Provides: {"impl_181"}
// Dependencies: {}
impl LlvmDwarfdump { # [doc = " Construct a new `llvm-dwarfdump` invocation. This assumes that `llvm-dwarfdump` is available"] # [doc = " at `$LLVM_BIN_DIR/llvm-dwarfdump`."] pub fn new () -> Self { let llvm_dwarfdump = llvm_bin_dir () . join ("llvm-dwarfdump") ; let cmd = Command :: new (llvm_dwarfdump) ; Self { cmd } } # [doc = " Provide an input file."] pub fn input < P : AsRef < Path > > (& mut self , path : P) -> & mut Self { self . cmd . arg (path . as_ref ()) ; self } }
};
}
