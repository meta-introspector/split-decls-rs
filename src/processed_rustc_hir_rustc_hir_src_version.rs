/* FP:version.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_version_USE_0001
/* FP:version.rs-0002 */ use std :: borrow :: Cow ;
/* FP:version.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_version_USE_0002
/* FP:version.rs-0004 */ use std :: fmt :: { self , Display } ;
/* FP:version.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_version_USE_0003
/* FP:version.rs-0006 */ use std :: sync :: OnceLock ;
/* FP:version.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_version_USE_0004
/* FP:version.rs-0008 */ use crate :: rustc_error_messages :: { DiagArgValue , IntoDiagArg } ;
/* FP:version.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_version_USE_0005
/* FP:version.rs-0010 */ use rustc_macros :: { Decodable , Encodable , HashStable_Generic , PrintAttribute , current_rustc_version , } ;
/* FP:version.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_version_USE_0006
/* FP:version.rs-0012 */ use crate :: attrs :: PrintAttribute ;
/* FP:version.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_version_STRUCT_0007
/* FP:version.rs-0014 */ # [derive (Encodable , Decodable , Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub struct RustcVersion { pub major : u16 , pub minor : u16 , pub patch : u16 , }
/* FP:version.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_version_IMPL_0008
/* FP:version.rs-0016 */ impl RustcVersion { pub const CURRENT : Self = current_rustc_version ! () ; pub fn current_overridable () -> Self { * CURRENT_OVERRIDABLE . get_or_init (| | { if let Ok (override_var) = std :: env :: var ("RUSTC_OVERRIDE_VERSION_STRING") && let Some (override_) = Self :: parse_str (& override_var) { override_ } else { Self :: CURRENT } }) } fn parse_str (value : & str) -> Option < Self > { let mut components = value . split ('-') . next () . unwrap () . splitn (3 , '.') ; let major = components . next () ? . parse () . ok () ? ; let minor = components . next () ? . parse () . ok () ? ; let patch = components . next () . unwrap_or ("0") . parse () . ok () ? ; Some (RustcVersion { major , minor , patch }) } }
/* FP:version.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_version_STATIC_0009
/* FP:version.rs-0018 */ static CURRENT_OVERRIDABLE : OnceLock < RustcVersion > = OnceLock :: new () ;
/* FP:version.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_version_IMPL_0010
/* FP:version.rs-0020 */ impl Display for RustcVersion { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (formatter , "{}.{}.{}" , self . major , self . minor , self . patch) } }
/* FP:version.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_version_IMPL_0011
/* FP:version.rs-0022 */ impl IntoDiagArg for RustcVersion { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string ())) } }