// Generated macro for impl_185 (impl)
macro_rules! Depcrate_external_deps_llvmimpl_185 {
() => {
// Module: crate::external_deps::llvm
// Provides: {"impl_185"}
// Dependencies: {}
impl LlvmDis { # [doc = " Construct a new `llvm-dis` invocation. This assumes that `llvm-dis` is available"] # [doc = " at `$LLVM_BIN_DIR/llvm-dis`."] pub fn new () -> Self { let llvm_dis = llvm_bin_dir () . join ("llvm-dis") ; let cmd = Command :: new (llvm_dis) ; Self { cmd } } # [doc = " Provide an input file."] pub fn input < P : AsRef < Path > > (& mut self , path : P) -> & mut Self { self . cmd . arg (path . as_ref ()) ; self } }
};
}
