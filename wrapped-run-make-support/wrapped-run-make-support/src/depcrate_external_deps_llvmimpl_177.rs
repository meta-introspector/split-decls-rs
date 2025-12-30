// Generated macro for impl_177 (impl)
macro_rules! Depcrate_external_deps_llvmimpl_177 {
() => {
// Module: crate::external_deps::llvm
// Provides: {"impl_177"}
// Dependencies: {}
impl LlvmObjdump { # [doc = " Construct a new `llvm-objdump` invocation. This assumes that `llvm-objdump` is available"] # [doc = " at `$LLVM_BIN_DIR/llvm-objdump`."] pub fn new () -> Self { let llvm_objdump = llvm_bin_dir () . join ("llvm-objdump") ; let cmd = Command :: new (llvm_objdump) ; Self { cmd } } # [doc = " Provide an input file."] pub fn input < P : AsRef < Path > > (& mut self , path : P) -> & mut Self { self . cmd . arg (path . as_ref ()) ; self } # [doc = " Disassemble all executable sections found in the input files."] pub fn disassemble (& mut self) -> & mut Self { self . cmd . arg ("-d") ; self } }
};
}
