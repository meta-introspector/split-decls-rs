// Generated macro for impl_183 (impl)
macro_rules! Depcrate_external_deps_llvmimpl_183 {
() => {
// Module: crate::external_deps::llvm
// Provides: {"impl_183"}
// Dependencies: {}
impl LlvmObjcopy { # [doc = " Construct a new `llvm-objcopy` invocation. This assumes that `llvm-objcopy` is available"] # [doc = " at `$LLVM_BIN_DIR/llvm-objcopy`."] pub fn new () -> Self { let llvm_objcopy = llvm_bin_dir () . join ("llvm-objcopy") ; let cmd = Command :: new (llvm_objcopy) ; Self { cmd } } # [doc = " Dump the contents of `section` into the file at `path`."] # [track_caller] pub fn dump_section < S : AsRef < str > , P : AsRef < Path > > (& mut self , section_name : S , path : P ,) -> & mut Self { self . cmd . arg ("--dump-section") ; self . cmd . arg (format ! ("{}={}" , section_name . as_ref () , path . as_ref () . to_str () . unwrap ())) ; self } }
};
}
