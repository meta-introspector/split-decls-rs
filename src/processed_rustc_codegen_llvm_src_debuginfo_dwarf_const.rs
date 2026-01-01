/* FP:dwarf_const.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_dwarf_const_USE_0001
/* FP:dwarf_const.rs-0002 */ use libc :: c_uint ;
/* FP:dwarf_const.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_dwarf_const_MACRO_0002
/* FP:dwarf_const.rs-0004 */ # [doc = " Helper macro to let us redeclare gimli's constants as our own constants"] # [doc = " with a different type, with less risk of copy-paste errors."] macro_rules ! declare_constant { ($ name : ident : $ type : ty) => { # [allow (non_upper_case_globals)] pub (crate) const $ name : $ type = :: gimli :: constants ::$ name . 0 as $ type ; const _ : () = assert ! ($ name as i128 == :: gimli :: constants ::$ name . 0 as i128) ; } ; }
/* FP:dwarf_const.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_dwarf_const_MACRO_0003
/* FP:dwarf_const.rs-0006 */ declare_constant ! (DW_TAG_const_type : c_uint) ;
/* FP:dwarf_const.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_dwarf_const_MACRO_0004
/* FP:dwarf_const.rs-0008 */ declare_constant ! (DW_LANG_Rust : c_uint) ;
/* FP:dwarf_const.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_dwarf_const_MACRO_0005
/* FP:dwarf_const.rs-0010 */ declare_constant ! (DW_ATE_boolean : c_uint) ;
/* FP:dwarf_const.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_dwarf_const_MACRO_0006
/* FP:dwarf_const.rs-0012 */ declare_constant ! (DW_ATE_float : c_uint) ;
/* FP:dwarf_const.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_dwarf_const_MACRO_0007
/* FP:dwarf_const.rs-0014 */ declare_constant ! (DW_ATE_signed : c_uint) ;
/* FP:dwarf_const.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_dwarf_const_MACRO_0008
/* FP:dwarf_const.rs-0016 */ declare_constant ! (DW_ATE_unsigned : c_uint) ;
/* FP:dwarf_const.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_dwarf_const_MACRO_0009
/* FP:dwarf_const.rs-0018 */ declare_constant ! (DW_ATE_UTF : c_uint) ;
/* FP:dwarf_const.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_dwarf_const_MACRO_0010
/* FP:dwarf_const.rs-0020 */ declare_constant ! (DW_OP_deref : u64) ;
/* FP:dwarf_const.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_dwarf_const_MACRO_0011
/* FP:dwarf_const.rs-0022 */ declare_constant ! (DW_OP_plus_uconst : u64) ;
/* FP:dwarf_const.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_dwarf_const_CONST_0012
/* FP:dwarf_const.rs-0024 */ # [doc = " Defined by LLVM in `llvm/include/llvm/BinaryFormat/Dwarf.h`."] # [doc = " Double-checked by a static assertion in `RustWrapper.cpp`."] # [allow (non_upper_case_globals)] pub (crate) const DW_OP_LLVM_fragment : u64 = 0x1000 ;