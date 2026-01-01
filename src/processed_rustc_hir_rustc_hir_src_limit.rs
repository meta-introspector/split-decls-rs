/* FP:limit.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_limit_USE_0001
/* FP:limit.rs-0002 */ use std :: fmt ;
/* FP:limit.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_limit_USE_0002
/* FP:limit.rs-0004 */ use std :: ops :: { Div , Mul } ;
/* FP:limit.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_limit_USE_0003
/* FP:limit.rs-0006 */ use crate :: rustc_error_messages :: { DiagArgValue , IntoDiagArg } ;
/* FP:limit.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_limit_USE_0004
/* FP:limit.rs-0008 */ use rustc_macros :: { Decodable , Encodable , HashStable_Generic } ;
/* FP:limit.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_limit_STRUCT_0005
/* FP:limit.rs-0010 */ # [doc = " New-type wrapper around `usize` for representing limits. Ensures that comparisons against"] # [doc = " limits are consistent throughout the compiler."] # [derive (Clone , Copy , Debug , HashStable_Generic , Encodable , Decodable)] pub struct Limit (pub usize) ;
/* FP:limit.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_limit_IMPL_0006
/* FP:limit.rs-0012 */ impl Limit { # [doc = " Create a new limit from a `usize`."] pub fn new (value : usize) -> Self { Limit (value) } # [doc = " Create a new unlimited limit."] pub fn unlimited () -> Self { Limit (usize :: MAX) } # [doc = " Check that `value` is within the limit. Ensures that the same comparisons are used"] # [doc = " throughout the compiler, as mismatches can cause ICEs, see #72540."] # [inline] pub fn value_within_limit (& self , value : usize) -> bool { value <= self . 0 } }
/* FP:limit.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_limit_IMPL_0007
/* FP:limit.rs-0014 */ impl From < usize > for Limit { fn from (value : usize) -> Self { Self :: new (value) } }
/* FP:limit.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_limit_IMPL_0008
/* FP:limit.rs-0016 */ impl fmt :: Display for Limit { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
/* FP:limit.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_limit_IMPL_0009
/* FP:limit.rs-0018 */ impl Div < usize > for Limit { type Output = Limit ; fn div (self , rhs : usize) -> Self :: Output { Limit :: new (self . 0 / rhs) } }
/* FP:limit.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_limit_IMPL_0010
/* FP:limit.rs-0020 */ impl Mul < usize > for Limit { type Output = Limit ; fn mul (self , rhs : usize) -> Self :: Output { Limit :: new (self . 0 * rhs) } }
/* FP:limit.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_limit_IMPL_0011
/* FP:limit.rs-0022 */ impl IntoDiagArg for Limit { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_string () . into_diag_arg (& mut None) } }