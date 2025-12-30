// Generated macro for impl_184 (impl)
macro_rules! Depcrate_external_deps_llvmimpl_184 {
() => {
// Module: crate::external_deps::llvm
// Provides: {"impl_184"}
// Dependencies: {}
impl LlvmAs { # [doc = " Construct a new `llvm-as` invocation. This assumes that `llvm-as` is available"] # [doc = " at `$LLVM_BIN_DIR/llvm-as`."] pub fn new () -> Self { let llvm_as = llvm_bin_dir () . join ("llvm-as") ; let cmd = Command :: new (llvm_as) ; Self { cmd } } # [doc = " Provide an input file."] pub fn input < P : AsRef < Path > > (& mut self , path : P) -> & mut Self { self . cmd . arg (path . as_ref ()) ; self } }
};
}
