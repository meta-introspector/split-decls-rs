/* FP:value.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_value_USE_0001
/* FP:value.rs-0002 */ use std :: hash :: { Hash , Hasher } ;
/* FP:value.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_value_USE_0002
/* FP:value.rs-0004 */ use std :: { fmt , ptr } ;
/* FP:value.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_value_USE_0003
/* FP:value.rs-0006 */ use crate :: llvm ;
/* FP:value.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_value_USE_0004
/* FP:value.rs-0008 */ pub (crate) use crate :: llvm :: Value ;
/* FP:value.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_value_IMPL_0005
/* FP:value.rs-0010 */ impl PartialEq for Value { fn eq (& self , other : & Self) -> bool { ptr :: eq (self , other) } }
/* FP:value.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_value_IMPL_0006
/* FP:value.rs-0012 */ impl Eq for Value { }
/* FP:value.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_value_IMPL_0007
/* FP:value.rs-0014 */ impl Hash for Value { fn hash < H : Hasher > (& self , hasher : & mut H) { (self as * const Self) . hash (hasher) ; } }
/* FP:value.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_value_IMPL_0008
/* FP:value.rs-0016 */ impl fmt :: Debug for Value { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& llvm :: build_string (| s | unsafe { llvm :: LLVMRustWriteValueToString (self , s) ; }) . expect ("non-UTF8 value description from LLVM") ,) } }