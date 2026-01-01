/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_USE_0001
/* FP:errors.rs-0002 */ use std :: ffi :: CString ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_USE_0002
/* FP:errors.rs-0004 */ use std :: path :: Path ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_USE_0003
/* FP:errors.rs-0006 */ use crate :: rustc_data_structures :: small_c_str :: SmallCStr ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_USE_0004
/* FP:errors.rs-0008 */ use crate :: rustc_complete :: { Diag , DiagCtxtHandle , Diagnostic , EmissionGuarantee , Level } ;
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_USE_0005
/* FP:errors.rs-0010 */ use rustc_macros :: Diagnostic ;
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_USE_0006
/* FP:errors.rs-0012 */ use crate :: rustc_complete :: Span ;
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_USE_0007
/* FP:errors.rs-0014 */ use crate :: fluent_generated as fluent ;
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0008
/* FP:errors.rs-0016 */ # [derive (Diagnostic)] # [diag (codegen_llvm_symbol_already_defined)] pub (crate) struct SymbolAlreadyDefined < 'a > { # [primary_span] pub span : Span , pub symbol_name : & 'a str , }
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0009
/* FP:errors.rs-0018 */ # [derive (Diagnostic)] # [diag (codegen_llvm_sanitizer_memtag_requires_mte)] pub (crate) struct SanitizerMemtagRequiresMte ;
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0010
/* FP:errors.rs-0020 */ pub (crate) struct ParseTargetMachineConfig < 'a > (pub LlvmError < 'a >) ;
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_IMPL_0011
/* FP:errors.rs-0022 */ impl < G : EmissionGuarantee > Diagnostic < '_ , G > for ParseTargetMachineConfig < '_ > { fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let diag : Diag < '_ , G > = self . 0 . into_diag (dcx , level) ; let (message , _) = diag . messages . first () . expect ("`LlvmError` with no message") ; let message = dcx . eagerly_translate_to_string (message . clone () , diag . args . iter ()) ; Diag :: new (dcx , level , fluent :: codegen_llvm_parse_target_machine_config) . with_arg ("error" , message) } }
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0012
/* FP:errors.rs-0024 */ # [derive (Diagnostic)] # [diag (codegen_llvm_autodiff_without_enable)] pub (crate) struct AutoDiffWithoutEnable ;
/* FP:errors.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0013
/* FP:errors.rs-0026 */ # [derive (Diagnostic)] # [diag (codegen_llvm_lto_bitcode_from_rlib)] pub (crate) struct LtoBitcodeFromRlib { pub err : String , }
/* FP:errors.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_ENUM_0014
/* FP:errors.rs-0028 */ # [derive (Diagnostic)] pub enum LlvmError < 'a > { # [diag (codegen_llvm_write_output)] WriteOutput { path : & 'a Path } , # [diag (codegen_llvm_target_machine)] CreateTargetMachine { triple : SmallCStr } , # [diag (codegen_llvm_run_passes)] RunLlvmPasses , # [diag (codegen_llvm_serialize_module)] SerializeModule { name : & 'a str } , # [diag (codegen_llvm_write_ir)] WriteIr { path : & 'a Path } , # [diag (codegen_llvm_prepare_thin_lto_context)] PrepareThinLtoContext , # [diag (codegen_llvm_load_bitcode)] LoadBitcode { name : CString } , # [diag (codegen_llvm_write_thinlto_key)] WriteThinLtoKey { err : std :: io :: Error } , # [diag (codegen_llvm_prepare_thin_lto_module)] PrepareThinLtoModule , # [diag (codegen_llvm_parse_bitcode)] ParseBitcode , # [diag (codegen_llvm_prepare_autodiff)] PrepareAutoDiff { src : String , target : String , error : String } , }
/* FP:errors.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0015
/* FP:errors.rs-0030 */ pub (crate) struct WithLlvmError < 'a > (pub LlvmError < 'a > , pub String) ;
/* FP:errors.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_IMPL_0016
/* FP:errors.rs-0032 */ impl < G : EmissionGuarantee > Diagnostic < '_ , G > for WithLlvmError < '_ > { fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { use LlvmError :: * ; let msg_with_llvm_err = match & self . 0 { WriteOutput { .. } => fluent :: codegen_llvm_write_output_with_llvm_err , CreateTargetMachine { .. } => fluent :: codegen_llvm_target_machine_with_llvm_err , RunLlvmPasses => fluent :: codegen_llvm_run_passes_with_llvm_err , SerializeModule { .. } => fluent :: codegen_llvm_serialize_module_with_llvm_err , WriteIr { .. } => fluent :: codegen_llvm_write_ir_with_llvm_err , PrepareThinLtoContext => fluent :: codegen_llvm_prepare_thin_lto_context_with_llvm_err , LoadBitcode { .. } => fluent :: codegen_llvm_load_bitcode_with_llvm_err , WriteThinLtoKey { .. } => fluent :: codegen_llvm_write_thinlto_key_with_llvm_err , PrepareThinLtoModule => fluent :: codegen_llvm_prepare_thin_lto_module_with_llvm_err , ParseBitcode => fluent :: codegen_llvm_parse_bitcode_with_llvm_err , PrepareAutoDiff { .. } => fluent :: codegen_llvm_prepare_autodiff_with_llvm_err , } ; self . 0 . into_diag (dcx , level) . with_primary_message (msg_with_llvm_err) . with_arg ("llvm_err" , self . 1) } }
/* FP:errors.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0017
/* FP:errors.rs-0034 */ # [derive (Diagnostic)] # [diag (codegen_llvm_from_llvm_optimization_diag)] pub (crate) struct FromLlvmOptimizationDiag < 'a > { pub filename : & 'a str , pub line : std :: ffi :: c_uint , pub column : std :: ffi :: c_uint , pub pass_name : & 'a str , pub kind : & 'a str , pub message : & 'a str , }
/* FP:errors.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0018
/* FP:errors.rs-0036 */ # [derive (Diagnostic)] # [diag (codegen_llvm_from_llvm_diag)] pub (crate) struct FromLlvmDiag { pub message : String , }
/* FP:errors.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0019
/* FP:errors.rs-0038 */ # [derive (Diagnostic)] # [diag (codegen_llvm_write_bytecode)] pub (crate) struct WriteBytecode < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
/* FP:errors.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0020
/* FP:errors.rs-0040 */ # [derive (Diagnostic)] # [diag (codegen_llvm_copy_bitcode)] pub (crate) struct CopyBitcode { pub err : std :: io :: Error , }
/* FP:errors.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0021
/* FP:errors.rs-0042 */ # [derive (Diagnostic)] # [diag (codegen_llvm_unknown_debuginfo_compression)] pub (crate) struct UnknownCompression { pub algorithm : & 'static str , }
/* FP:errors.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0022
/* FP:errors.rs-0044 */ # [derive (Diagnostic)] # [diag (codegen_llvm_mismatch_data_layout)] pub (crate) struct MismatchedDataLayout < 'a > { pub rustc_target : & 'a str , pub rustc_layout : & 'a str , pub llvm_target : & 'a str , pub llvm_layout : & 'a str , }
/* FP:errors.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0023
/* FP:errors.rs-0046 */ # [derive (Diagnostic)] # [diag (codegen_llvm_fixed_x18_invalid_arch)] pub (crate) struct FixedX18InvalidArch < 'a > { pub arch : & 'a str , }
/* FP:errors.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_errors_STRUCT_0024
/* FP:errors.rs-0048 */ # [derive (Diagnostic)] # [diag (codegen_llvm_sanitizer_kcfi_arity_requires_llvm_21_0_0)] pub (crate) struct SanitizerKcfiArityRequiresLLVM2100 ;