/* FP:gdb.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_USE_0001
/* FP:gdb.rs-0002 */ use crate :: rustc_codegen_ssa :: base :: collect_debugger_visualizers_transitive ;
/* FP:gdb.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_USE_0002
/* FP:gdb.rs-0004 */ use crate :: rustc_codegen_ssa :: traits :: * ;
/* FP:gdb.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_USE_0003
/* FP:gdb.rs-0006 */ use crate :: rustc_complete :: def_id :: LOCAL_CRATE ;
/* FP:gdb.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_USE_0004
/* FP:gdb.rs-0008 */ use crate :: rustc_complete :: bug ;
/* FP:gdb.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_USE_0005
/* FP:gdb.rs-0010 */ use crate :: rustc_complete :: middle :: debugger_visualizer :: DebuggerVisualizerType ;
/* FP:gdb.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_USE_0006
/* FP:gdb.rs-0012 */ use crate :: rustc_complete :: config :: { CrateType , DebugInfo } ;
/* FP:gdb.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_USE_0007
/* FP:gdb.rs-0014 */ use crate :: builder :: Builder ;
/* FP:gdb.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_USE_0008
/* FP:gdb.rs-0016 */ use crate :: common :: CodegenCx ;
/* FP:gdb.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_USE_0009
/* FP:gdb.rs-0018 */ use crate :: llvm ;
/* FP:gdb.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_USE_0010
/* FP:gdb.rs-0020 */ use crate :: value :: Value ;
/* FP:gdb.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_FN_0011
/* FP:gdb.rs-0022 */ # [doc = " Inserts a side-effect free instruction sequence that makes sure that the"] # [doc = " .debug_gdb_scripts global is referenced, so it isn't removed by the linker."] pub (crate) fn insert_reference_to_gdb_debug_scripts_section_global (bx : & mut Builder < '_ , '_ , '_ >) { if needs_gdb_debug_scripts_section (bx) { let gdb_debug_scripts_section = get_or_insert_gdb_debug_scripts_section_global (bx) ; let volatile_load_instruction = bx . volatile_load (bx . type_i8 () , gdb_debug_scripts_section) ; unsafe { llvm :: LLVMSetAlignment (volatile_load_instruction , 1) ; } } }
/* FP:gdb.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_FN_0012
/* FP:gdb.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_gdb_FN_0013
/* FP:gdb.rs-0026 */ pub (crate) fn needs_gdb_debug_scripts_section (cx : & CodegenCx < '_ , '_ >) -> bool { let embed_visualizers = cx . tcx . crate_types () . iter () . any (| & crate_type | match crate_type { CrateType :: Executable | CrateType :: Dylib | CrateType :: Cdylib | CrateType :: Staticlib | CrateType :: Sdylib => { true } CrateType :: ProcMacro => { false } CrateType :: Rlib => { false } }) ; cx . sess () . opts . debuginfo != DebugInfo :: None && cx . sess () . target . emit_debug_gdb_scripts && embed_visualizers }