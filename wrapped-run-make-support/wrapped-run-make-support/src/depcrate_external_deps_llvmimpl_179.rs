// Generated macro for impl_179 (impl)
macro_rules! Depcrate_external_deps_llvmimpl_179 {
() => {
// Module: crate::external_deps::llvm
// Provides: {"impl_179"}
// Dependencies: {}
impl LlvmNm { # [doc = " Construct a new `llvm-nm` invocation. This assumes that `llvm-nm` is available"] # [doc = " at `$LLVM_BIN_DIR/llvm-nm`."] pub fn new () -> Self { let llvm_nm = llvm_bin_dir () . join ("llvm-nm") ; let cmd = Command :: new (llvm_nm) ; Self { cmd } } # [doc = " Provide an input file."] pub fn input < P : AsRef < Path > > (& mut self , path : P) -> & mut Self { self . cmd . arg (path . as_ref ()) ; self } }
};
}
