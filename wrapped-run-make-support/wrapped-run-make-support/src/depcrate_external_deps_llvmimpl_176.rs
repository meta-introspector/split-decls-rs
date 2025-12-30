// Generated macro for impl_176 (impl)
macro_rules! Depcrate_external_deps_llvmimpl_176 {
() => {
// Module: crate::external_deps::llvm
// Provides: {"impl_176"}
// Dependencies: {}
impl LlvmFilecheck { # [doc = " Construct a new `llvm-filecheck` invocation. This assumes that `llvm-filecheck` is available"] # [doc = " at `$LLVM_FILECHECK`."] # [track_caller] pub fn new () -> Self { let llvm_filecheck = env_var ("LLVM_FILECHECK") ; let cmd = Command :: new (llvm_filecheck) ; Self { cmd } } # [doc = " Provide a buffer representing standard input containing patterns that will be matched"] # [doc = " against the `.patterns(path)` call."] pub fn stdin_buf < I : AsRef < [u8] > > (& mut self , input : I) -> & mut Self { self . cmd . stdin_buf (input) ; self } # [doc = " Provide the patterns that need to be matched."] pub fn patterns < P : AsRef < Path > > (& mut self , path : P) -> & mut Self { self . cmd . arg (path . as_ref ()) ; self } # [doc = " `--input-file` option."] pub fn input_file < P : AsRef < Path > > (& mut self , input_file : P) -> & mut Self { self . cmd . arg ("--input-file") ; self . cmd . arg (input_file . as_ref ()) ; self } }
};
}
