/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_USE_0001
/* FP:errors.rs-0002 */ use std :: io ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_USE_0002
/* FP:errors.rs-0004 */ use std :: path :: Path ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_USE_0003
/* FP:errors.rs-0006 */ use rustc_macros :: Diagnostic ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_USE_0004
/* FP:errors.rs-0008 */ use crate :: rustc_complete :: { Span , Symbol } ;
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0005
/* FP:errors.rs-0010 */ # [derive (Diagnostic)] # [diag (interface_crate_name_does_not_match)] pub (crate) struct CrateNameDoesNotMatch { # [primary_span] pub (crate) span : Span , pub (crate) crate_name : Symbol , pub (crate) attr_crate_name : Symbol , }
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0006
/* FP:errors.rs-0012 */ # [derive (Diagnostic)] # [diag (interface_crate_name_invalid)] pub (crate) struct CrateNameInvalid < 'a > { pub (crate) crate_name : & 'a str , }
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0007
/* FP:errors.rs-0014 */ # [derive (Diagnostic)] # [diag (interface_ferris_identifier)] pub struct FerrisIdentifier { # [primary_span] pub spans : Vec < Span > , # [suggestion (code = "{ferris_fix}" , applicability = "maybe-incorrect")] pub first_span : Span , pub ferris_fix : & 'static str , }
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0008
/* FP:errors.rs-0016 */ # [derive (Diagnostic)] # [diag (interface_emoji_identifier)] pub struct EmojiIdentifier { # [primary_span] pub spans : Vec < Span > , pub ident : Symbol , }
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0009
/* FP:errors.rs-0018 */ # [derive (Diagnostic)] # [diag (interface_mixed_bin_crate)] pub struct MixedBinCrate ;
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0010
/* FP:errors.rs-0020 */ # [derive (Diagnostic)] # [diag (interface_mixed_proc_macro_crate)] pub struct MixedProcMacroCrate ;
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0011
/* FP:errors.rs-0022 */ # [derive (Diagnostic)] # [diag (interface_error_writing_dependencies)] pub struct ErrorWritingDependencies < 'a > { pub path : & 'a Path , pub error : io :: Error , }
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0012
/* FP:errors.rs-0024 */ # [derive (Diagnostic)] # [diag (interface_input_file_would_be_overwritten)] pub struct InputFileWouldBeOverWritten < 'a > { pub path : & 'a Path , }
/* FP:errors.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0013
/* FP:errors.rs-0026 */ # [derive (Diagnostic)] # [diag (interface_generated_file_conflicts_with_directory)] pub struct GeneratedFileConflictsWithDirectory < 'a > { pub input_path : & 'a Path , pub dir_path : & 'a Path , }
/* FP:errors.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0014
/* FP:errors.rs-0028 */ # [derive (Diagnostic)] # [diag (interface_temps_dir_error)] pub struct TempsDirError ;
/* FP:errors.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0015
/* FP:errors.rs-0030 */ # [derive (Diagnostic)] # [diag (interface_out_dir_error)] pub struct OutDirError ;
/* FP:errors.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0016
/* FP:errors.rs-0032 */ # [derive (Diagnostic)] # [diag (interface_failed_writing_file)] pub struct FailedWritingFile < 'a > { pub path : & 'a Path , pub error : io :: Error , }
/* FP:errors.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0017
/* FP:errors.rs-0034 */ # [derive (Diagnostic)] # [diag (interface_proc_macro_crate_panic_abort)] pub struct ProcMacroCratePanicAbort ;
/* FP:errors.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0018
/* FP:errors.rs-0036 */ # [derive (Diagnostic)] # [diag (interface_multiple_output_types_adaption)] pub struct MultipleOutputTypesAdaption ;
/* FP:errors.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0019
/* FP:errors.rs-0038 */ # [derive (Diagnostic)] # [diag (interface_ignoring_extra_filename)] pub struct IgnoringExtraFilename ;
/* FP:errors.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0020
/* FP:errors.rs-0040 */ # [derive (Diagnostic)] # [diag (interface_ignoring_out_dir)] pub struct IgnoringOutDir ;
/* FP:errors.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0021
/* FP:errors.rs-0042 */ # [derive (Diagnostic)] # [diag (interface_multiple_output_types_to_stdout)] pub struct MultipleOutputTypesToStdout ;
/* FP:errors.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_errors_STRUCT_0022
/* FP:errors.rs-0044 */ # [derive (Diagnostic)] # [diag (interface_abi_required_feature)] # [note] # [note (interface_abi_required_feature_issue)] pub (crate) struct AbiRequiredTargetFeature < 'a > { pub feature : & 'a str , pub enabled : & 'a str , }