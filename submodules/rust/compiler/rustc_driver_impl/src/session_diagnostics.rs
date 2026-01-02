mkuse!{use std :: error :: Error ;}
mkuse!{use rustc_macros :: { Diagnostic , Subdiagnostic } ;}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_cant_emit_mir)] pub struct CantEmitMIR { pub error : std :: io :: Error , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_rlink_unable_to_read)] pub (crate) struct RlinkUnableToRead { pub err : std :: io :: Error , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_rlink_wrong_file_type)] pub (crate) struct RLinkWrongFileType ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_rlink_empty_version_number)] pub (crate) struct RLinkEmptyVersionNumber ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_rlink_encoding_version_mismatch)] pub (crate) struct RLinkEncodingVersionMismatch { pub version_array : String , pub rlink_version : u32 , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_rlink_rustc_version_mismatch)] pub (crate) struct RLinkRustcVersionMismatch < 'a > { pub rustc_version : String , pub current_version : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_rlink_no_a_file)] pub (crate) struct RlinkNotAFile ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_rlink_corrupt_file)] pub (crate) struct RlinkCorruptFile < 'a > { pub file : & 'a std :: path :: Path , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_ice)] pub (crate) struct Ice ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_ice_bug_report)] pub (crate) struct IceBugReport < 'a > { pub bug_report_url : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_ice_bug_report_update_note)] pub (crate) struct UpdateNightlyNote ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_ice_bug_report_internal_feature)] pub (crate) struct IceBugReportInternalFeature ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_ice_version)] pub (crate) struct IceVersion < 'a > { pub version : & 'a str , pub triple : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_ice_path)] pub (crate) struct IcePath { pub path : std :: path :: PathBuf , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_ice_path_error)] pub (crate) struct IcePathError { pub path : std :: path :: PathBuf , pub error : String , # [subdiagnostic] pub env_var : Option < IcePathErrorEnv > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (driver_impl_ice_path_error_env)] pub (crate) struct IcePathErrorEnv { pub env_var : std :: path :: PathBuf , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_ice_flags)] pub (crate) struct IceFlags { pub flags : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_ice_exclude_cargo_defaults)] pub (crate) struct IceExcludeCargoDefaults ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (driver_impl_unstable_feature_usage)] pub (crate) struct UnstableFeatureUsage { pub error : Box < dyn Error > , }}}