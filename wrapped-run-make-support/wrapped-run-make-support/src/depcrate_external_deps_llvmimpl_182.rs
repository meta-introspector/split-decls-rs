// Generated macro for impl_182 (impl)
macro_rules! Depcrate_external_deps_llvmimpl_182 {
() => {
// Module: crate::external_deps::llvm
// Provides: {"impl_182"}
// Dependencies: {}
impl LlvmPdbutil { # [doc = " Construct a new `llvm-pdbutil` invocation. This assumes that `llvm-pdbutil` is available"] # [doc = " at `$LLVM_BIN_DIR/llvm-pdbutil`."] pub fn new () -> Self { let llvm_pdbutil = llvm_bin_dir () . join ("llvm-pdbutil") ; let cmd = Command :: new (llvm_pdbutil) ; Self { cmd } } # [doc = " Provide an input file."] pub fn input < P : AsRef < Path > > (& mut self , path : P) -> & mut Self { self . cmd . arg (path . as_ref ()) ; self } }
};
}
