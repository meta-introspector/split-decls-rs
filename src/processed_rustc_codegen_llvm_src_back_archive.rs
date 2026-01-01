/* FP:archive.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_archive_USE_0001
/* FP:archive.rs-0002 */ use std :: ffi :: { CStr , c_char , c_void } ;
/* FP:archive.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_archive_USE_0002
/* FP:archive.rs-0004 */ use std :: io ;
/* FP:archive.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_archive_USE_0003
/* FP:archive.rs-0006 */ use crate :: rustc_codegen_ssa :: back :: archive :: { ArArchiveBuilder , ArchiveBuilder , ArchiveBuilderBuilder , DEFAULT_OBJECT_READER , ObjectReader , } ;
/* FP:archive.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_archive_USE_0004
/* FP:archive.rs-0008 */ use crate :: rustc_complete :: Session ;
/* FP:archive.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_archive_USE_0005
/* FP:archive.rs-0010 */ use crate :: llvm ;
/* FP:archive.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_archive_STRUCT_0006
/* FP:archive.rs-0012 */ pub (crate) struct LlvmArchiveBuilderBuilder ;
/* FP:archive.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_archive_IMPL_0007
/* FP:archive.rs-0014 */ impl ArchiveBuilderBuilder for LlvmArchiveBuilderBuilder { fn new_archive_builder < 'a > (& self , sess : & 'a Session) -> Box < dyn ArchiveBuilder + 'a > { Box :: new (ArArchiveBuilder :: new (sess , & LLVM_OBJECT_READER)) } }
/* FP:archive.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_archive_STATIC_0008
/* FP:archive.rs-0016 */ static LLVM_OBJECT_READER : ObjectReader = ObjectReader { get_symbols : get_llvm_object_symbols , is_64_bit_object_file : llvm_is_64_bit_object_file , is_ec_object_file : llvm_is_ec_object_file , is_any_arm64_coff : llvm_is_any_arm64_coff , get_xcoff_member_alignment : DEFAULT_OBJECT_READER . get_xcoff_member_alignment , } ;
/* FP:archive.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_archive_FN_0009
/* FP:archive.rs-0018 */ # [deny (unsafe_op_in_unsafe_fn)] fn get_llvm_object_symbols (buf : & [u8] , f : & mut dyn FnMut (& [u8]) -> io :: Result < () > ,) -> io :: Result < bool > { let mut state = Box :: new (f) ; let err = unsafe { llvm :: LLVMRustGetSymbols (buf . as_ptr () , buf . len () , (& raw mut * state) as * mut c_void , callback , error_callback ,) } ; if err . is_null () { return Ok (true) ; } else { let error = unsafe { * Box :: from_raw (err as * mut io :: Error) } ; if buf . starts_with (& [0xDE , 0xCE , 0x17 , 0x0B]) || buf . starts_with (& [b'B' , b'C' , 0xC0 , 0xDE]) { eprintln ! ("warning: Failed to read symbol table from LLVM bitcode: {}" , error) ; return Ok (true) ; } else { return Err (error) ; } } unsafe extern "C" fn callback (state : * mut c_void , symbol_name : * const c_char) -> * mut c_void { let f = unsafe { & mut * (state as * mut & mut dyn FnMut (& [u8]) -> io :: Result < () >) } ; match f (unsafe { CStr :: from_ptr (symbol_name) } . to_bytes ()) { Ok (()) => std :: ptr :: null_mut () , Err (err) => Box :: into_raw (Box :: new (err) as Box < io :: Error >) as * mut c_void , } } unsafe extern "C" fn error_callback (error : * const c_char) -> * mut c_void { let error = unsafe { CStr :: from_ptr (error) } ; Box :: into_raw (Box :: new (io :: Error :: new (io :: ErrorKind :: Other , format ! ("LLVM error: {}" , error . to_string_lossy ()) ,)) as Box < io :: Error >) as * mut c_void } }
/* FP:archive.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_archive_FN_0010
/* FP:archive.rs-0020 */ fn llvm_is_64_bit_object_file (buf : & [u8]) -> bool { unsafe { llvm :: LLVMRustIs64BitSymbolicFile (buf . as_ptr () , buf . len ()) } }
/* FP:archive.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_archive_FN_0011
/* FP:archive.rs-0022 */ fn llvm_is_ec_object_file (buf : & [u8]) -> bool { unsafe { llvm :: LLVMRustIsECObject (buf . as_ptr () , buf . len ()) } }
/* FP:archive.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_archive_FN_0012
/* FP:archive.rs-0024 */ fn llvm_is_any_arm64_coff (buf : & [u8]) -> bool { unsafe { llvm :: LLVMRustIsAnyArm64Coff (buf . as_ptr () , buf . len ()) } }