// Generated macro for impl_178 (impl)
macro_rules! Depcrate_external_deps_llvmimpl_178 {
() => {
// Module: crate::external_deps::llvm
// Provides: {"impl_178"}
// Dependencies: {}
impl LlvmAr { # [doc = " Construct a new `llvm-ar` invocation. This assumes that `llvm-ar` is available"] # [doc = " at `$LLVM_BIN_DIR/llvm-ar`."] pub fn new () -> Self { let llvm_ar = llvm_bin_dir () . join ("llvm-ar") ; let cmd = Command :: new (llvm_ar) ; Self { cmd } } # [doc = " Automatically pass the commonly used arguments `rcus`, used for combining one or more"] # [doc = " input object files into one output static library file."] pub fn obj_to_ar (& mut self) -> & mut Self { self . cmd . arg ("rcus") ; self } # [doc = " Like `obj_to_ar` except creating a thin archive."] pub fn obj_to_thin_ar (& mut self) -> & mut Self { self . cmd . arg ("rcus") . arg ("--thin") ; self } # [doc = " Extract archive members back to files."] pub fn extract (& mut self) -> & mut Self { self . cmd . arg ("x") ; self } # [doc = " Print the table of contents."] pub fn table_of_contents (& mut self) -> & mut Self { self . cmd . arg ("t") ; self } # [doc = " Provide an output, then an input file. Bundled in one function, as llvm-ar has"] # [doc = " no \"--output\"-style flag."] pub fn output_input (& mut self , out : impl AsRef < Path > , input : impl AsRef < Path >) -> & mut Self { self . cmd . arg (out . as_ref ()) ; self . cmd . arg (input . as_ref ()) ; self } }
};
}
