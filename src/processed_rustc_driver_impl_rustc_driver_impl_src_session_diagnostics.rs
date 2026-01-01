/* FP:session_diagnostics.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_USE_0001
/* FP:session_diagnostics.rs-0002 */ use std :: error :: Error ;
/* FP:session_diagnostics.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_USE_0002
/* FP:session_diagnostics.rs-0004 */ use rustc_macros :: { Diagnostic , Subdiagnostic } ;
/* FP:session_diagnostics.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0003
/* FP:session_diagnostics.rs-0006 */ # [derive (Diagnostic)] # [diag (driver_impl_cant_emit_mir)] pub struct CantEmitMIR { pub error : std :: io :: Error , }
/* FP:session_diagnostics.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0004
/* FP:session_diagnostics.rs-0008 */ # [derive (Diagnostic)] # [diag (driver_impl_rlink_unable_to_read)] pub (crate) struct RlinkUnableToRead { pub err : std :: io :: Error , }
/* FP:session_diagnostics.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0005
/* FP:session_diagnostics.rs-0010 */ # [derive (Diagnostic)] # [diag (driver_impl_rlink_wrong_file_type)] pub (crate) struct RLinkWrongFileType ;
/* FP:session_diagnostics.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0006
/* FP:session_diagnostics.rs-0012 */ # [derive (Diagnostic)] # [diag (driver_impl_rlink_empty_version_number)] pub (crate) struct RLinkEmptyVersionNumber ;
/* FP:session_diagnostics.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0007
/* FP:session_diagnostics.rs-0014 */ # [derive (Diagnostic)] # [diag (driver_impl_rlink_encoding_version_mismatch)] pub (crate) struct RLinkEncodingVersionMismatch { pub version_array : String , pub rlink_version : u32 , }
/* FP:session_diagnostics.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0008
/* FP:session_diagnostics.rs-0016 */ # [derive (Diagnostic)] # [diag (driver_impl_rlink_rustc_version_mismatch)] pub (crate) struct RLinkRustcVersionMismatch < 'a > { pub rustc_version : String , pub current_version : & 'a str , }
/* FP:session_diagnostics.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0009
/* FP:session_diagnostics.rs-0018 */ # [derive (Diagnostic)] # [diag (driver_impl_rlink_no_a_file)] pub (crate) struct RlinkNotAFile ;
/* FP:session_diagnostics.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0010
/* FP:session_diagnostics.rs-0020 */ # [derive (Diagnostic)] # [diag (driver_impl_rlink_corrupt_file)] pub (crate) struct RlinkCorruptFile < 'a > { pub file : & 'a std :: path :: Path , }
/* FP:session_diagnostics.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0011
/* FP:session_diagnostics.rs-0022 */ # [derive (Diagnostic)] # [diag (driver_impl_ice)] pub (crate) struct Ice ;
/* FP:session_diagnostics.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0012
/* FP:session_diagnostics.rs-0024 */ # [derive (Diagnostic)] # [diag (driver_impl_ice_bug_report)] pub (crate) struct IceBugReport < 'a > { pub bug_report_url : & 'a str , }
/* FP:session_diagnostics.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0013
/* FP:session_diagnostics.rs-0026 */ # [derive (Diagnostic)] # [diag (driver_impl_ice_bug_report_update_note)] pub (crate) struct UpdateNightlyNote ;
/* FP:session_diagnostics.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0014
/* FP:session_diagnostics.rs-0028 */ # [derive (Diagnostic)] # [diag (driver_impl_ice_bug_report_internal_feature)] pub (crate) struct IceBugReportInternalFeature ;
/* FP:session_diagnostics.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0015
/* FP:session_diagnostics.rs-0030 */ # [derive (Diagnostic)] # [diag (driver_impl_ice_version)] pub (crate) struct IceVersion < 'a > { pub version : & 'a str , pub triple : & 'a str , }
/* FP:session_diagnostics.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0016
/* FP:session_diagnostics.rs-0032 */ # [derive (Diagnostic)] # [diag (driver_impl_ice_path)] pub (crate) struct IcePath { pub path : std :: path :: PathBuf , }
/* FP:session_diagnostics.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0017
/* FP:session_diagnostics.rs-0034 */ # [derive (Diagnostic)] # [diag (driver_impl_ice_path_error)] pub (crate) struct IcePathError { pub path : std :: path :: PathBuf , pub error : String , # [subdiagnostic] pub env_var : Option < IcePathErrorEnv > , }
/* FP:session_diagnostics.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0018
/* FP:session_diagnostics.rs-0036 */ # [derive (Subdiagnostic)] # [note (driver_impl_ice_path_error_env)] pub (crate) struct IcePathErrorEnv { pub env_var : std :: path :: PathBuf , }
/* FP:session_diagnostics.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0019
/* FP:session_diagnostics.rs-0038 */ # [derive (Diagnostic)] # [diag (driver_impl_ice_flags)] pub (crate) struct IceFlags { pub flags : String , }
/* FP:session_diagnostics.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0020
/* FP:session_diagnostics.rs-0040 */ # [derive (Diagnostic)] # [diag (driver_impl_ice_exclude_cargo_defaults)] pub (crate) struct IceExcludeCargoDefaults ;
/* FP:session_diagnostics.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_session_diagnostics_STRUCT_0021
/* FP:session_diagnostics.rs-0042 */ # [derive (Diagnostic)] # [diag (driver_impl_unstable_feature_usage)] pub (crate) struct UnstableFeatureUsage { pub error : Box < dyn Error > , }