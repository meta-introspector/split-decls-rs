// Generated macro for impl_180 (impl)
macro_rules! Depcrate_external_deps_llvmimpl_180 {
() => {
// Module: crate::external_deps::llvm
// Provides: {"impl_180"}
// Dependencies: {}
impl LlvmBcanalyzer { # [doc = " Construct a new `llvm-bcanalyzer` invocation. This assumes that `llvm-bcanalyzer` is available"] # [doc = " at `$LLVM_BIN_DIR/llvm-bcanalyzer`."] pub fn new () -> Self { let llvm_bcanalyzer = llvm_bin_dir () . join ("llvm-bcanalyzer") ; let cmd = Command :: new (llvm_bcanalyzer) ; Self { cmd } } # [doc = " Provide an input file."] pub fn input < P : AsRef < Path > > (& mut self , path : P) -> & mut Self { self . cmd . arg (path . as_ref ()) ; self } }
};
}
