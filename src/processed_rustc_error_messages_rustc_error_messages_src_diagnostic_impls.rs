/* FP:diagnostic_impls.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_USE_0001
/* FP:diagnostic_impls.rs-0002 */ use std :: backtrace :: Backtrace ;
/* FP:diagnostic_impls.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_USE_0002
/* FP:diagnostic_impls.rs-0004 */ use std :: borrow :: Cow ;
/* FP:diagnostic_impls.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_USE_0003
/* FP:diagnostic_impls.rs-0006 */ use std :: fmt ;
/* FP:diagnostic_impls.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_USE_0004
/* FP:diagnostic_impls.rs-0008 */ use std :: num :: ParseIntError ;
/* FP:diagnostic_impls.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_USE_0005
/* FP:diagnostic_impls.rs-0010 */ use std :: path :: { Path , PathBuf } ;
/* FP:diagnostic_impls.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_USE_0006
/* FP:diagnostic_impls.rs-0012 */ use std :: process :: ExitStatus ;
/* FP:diagnostic_impls.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_USE_0007
/* FP:diagnostic_impls.rs-0014 */ use rustc_ast as ast ;
/* FP:diagnostic_impls.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_USE_0008
/* FP:diagnostic_impls.rs-0016 */ use rustc_ast_pretty :: pprust ;
/* FP:diagnostic_impls.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_USE_0009
/* FP:diagnostic_impls.rs-0018 */ use crate :: rustc_complete :: edition :: Edition ;
/* FP:diagnostic_impls.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_USE_0010
/* FP:diagnostic_impls.rs-0020 */ use crate :: { DiagArgValue , IntoDiagArg } ;
/* FP:diagnostic_impls.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_STRUCT_0011
/* FP:diagnostic_impls.rs-0022 */ pub struct DiagArgFromDisplay < 'a > (pub & 'a dyn fmt :: Display) ;
/* FP:diagnostic_impls.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0012
/* FP:diagnostic_impls.rs-0024 */ impl IntoDiagArg for DiagArgFromDisplay < '_ > { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . 0 . to_string () . into_diag_arg (path) } }
/* FP:diagnostic_impls.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0013
/* FP:diagnostic_impls.rs-0026 */ impl < 'a > From < & 'a dyn fmt :: Display > for DiagArgFromDisplay < 'a > { fn from (t : & 'a dyn fmt :: Display) -> Self { DiagArgFromDisplay (t) } }
/* FP:diagnostic_impls.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0014
/* FP:diagnostic_impls.rs-0028 */ impl < 'a , T : fmt :: Display > From < & 'a T > for DiagArgFromDisplay < 'a > { fn from (t : & 'a T) -> Self { DiagArgFromDisplay (t) } }
/* FP:diagnostic_impls.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0015
/* FP:diagnostic_impls.rs-0030 */ impl < 'a , T : Clone + IntoDiagArg > IntoDiagArg for & 'a T { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . clone () . into_diag_arg (path) } }
/* FP:diagnostic_impls.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_MACRO_0016
/* FP:diagnostic_impls.rs-0032 */ # [macro_export] macro_rules ! into_diag_arg_using_display { ($ ($ ty : ty) ,+ $ (,) ?) => { $ (impl $ crate :: IntoDiagArg for $ ty { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> $ crate :: DiagArgValue { self . to_string () . into_diag_arg (path) } }) + } }
/* FP:diagnostic_impls.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_MACRO_0017
/* FP:diagnostic_impls.rs-0034 */ macro_rules ! into_diag_arg_for_number { ($ ($ ty : ty) ,+ $ (,) ?) => { $ (impl $ crate :: IntoDiagArg for $ ty { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> $ crate :: DiagArgValue { # [allow (irrefutable_let_patterns)] if let Ok (n) = TryInto ::< i32 >:: try_into (self) { $ crate :: DiagArgValue :: Number (n) } else { self . to_string () . into_diag_arg (path) } } }) + } }
/* FP:diagnostic_impls.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_MACRO_0018
/* FP:diagnostic_impls.rs-0036 */ into_diag_arg_using_display ! (ast :: ParamKindOrd , std :: io :: Error , Box < dyn std :: error :: Error >, std :: num :: NonZero < u32 >, Edition , crate :: rustc_span :: Ident , crate :: rustc_span :: MacroRulesNormalizedIdent , ParseIntError , ExitStatus ,) ;
/* FP:diagnostic_impls.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_MACRO_0019
/* FP:diagnostic_impls.rs-0038 */ into_diag_arg_for_number ! (i8 , u8 , i16 , u16 , i32 , u32 , i64 , u64 , i128 , u128 , isize , usize) ;
/* FP:diagnostic_impls.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0020
/* FP:diagnostic_impls.rs-0040 */ impl IntoDiagArg for bool { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { if self { DiagArgValue :: Str (Cow :: Borrowed ("true")) } else { DiagArgValue :: Str (Cow :: Borrowed ("false")) } } }
/* FP:diagnostic_impls.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0021
/* FP:diagnostic_impls.rs-0042 */ impl IntoDiagArg for char { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (format ! ("{self:?}"))) } }
/* FP:diagnostic_impls.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0022
/* FP:diagnostic_impls.rs-0044 */ impl IntoDiagArg for Vec < char > { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: StrListSepByAnd (self . into_iter () . map (| c | Cow :: Owned (format ! ("{c:?}"))) . collect () ,) } }
/* FP:diagnostic_impls.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0023
/* FP:diagnostic_impls.rs-0046 */ impl IntoDiagArg for crate :: rustc_span :: Symbol { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_ident_string () . into_diag_arg (path) } }
/* FP:diagnostic_impls.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0024
/* FP:diagnostic_impls.rs-0048 */ impl < 'a > IntoDiagArg for & 'a str { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_string () . into_diag_arg (path) } }
/* FP:diagnostic_impls.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0025
/* FP:diagnostic_impls.rs-0050 */ impl IntoDiagArg for String { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self)) } }
/* FP:diagnostic_impls.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0026
/* FP:diagnostic_impls.rs-0052 */ impl < 'a > IntoDiagArg for Cow < 'a , str > { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . into_owned ())) } }
/* FP:diagnostic_impls.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0027
/* FP:diagnostic_impls.rs-0054 */ impl < 'a > IntoDiagArg for & 'a Path { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . display () . to_string ())) } }
/* FP:diagnostic_impls.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0028
/* FP:diagnostic_impls.rs-0056 */ impl IntoDiagArg for PathBuf { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . display () . to_string ())) } }
/* FP:diagnostic_impls.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0029
/* FP:diagnostic_impls.rs-0058 */ impl IntoDiagArg for ast :: Expr { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (pprust :: expr_to_string (& self))) } }
/* FP:diagnostic_impls.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0030
/* FP:diagnostic_impls.rs-0060 */ impl IntoDiagArg for ast :: Path { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (pprust :: path_to_string (& self))) } }
/* FP:diagnostic_impls.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0031
/* FP:diagnostic_impls.rs-0062 */ impl IntoDiagArg for ast :: token :: Token { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (pprust :: token_to_string (& self)) } }
/* FP:diagnostic_impls.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0032
/* FP:diagnostic_impls.rs-0064 */ impl IntoDiagArg for ast :: token :: TokenKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (pprust :: token_kind_to_string (& self)) } }
/* FP:diagnostic_impls.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0033
/* FP:diagnostic_impls.rs-0066 */ impl IntoDiagArg for std :: ffi :: CString { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string_lossy () . into_owned ())) } }
/* FP:diagnostic_impls.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0034
/* FP:diagnostic_impls.rs-0068 */ impl IntoDiagArg for crate :: rustc_data_structures :: small_c_str :: SmallCStr { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string_lossy () . into_owned ())) } }
/* FP:diagnostic_impls.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0035
/* FP:diagnostic_impls.rs-0070 */ impl IntoDiagArg for ast :: Visibility { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { let s = pprust :: vis_to_string (& self) ; let s = s . trim_end () . to_string () ; DiagArgValue :: Str (Cow :: Owned (s)) } }
/* FP:diagnostic_impls.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0036
/* FP:diagnostic_impls.rs-0072 */ impl IntoDiagArg for Backtrace { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: from (self . to_string ())) } }
/* FP:diagnostic_impls.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0037
/* FP:diagnostic_impls.rs-0074 */ impl IntoDiagArg for ast :: util :: parser :: ExprPrecedence { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Number (self as i32) } }
/* FP:diagnostic_impls.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_error_messages_src_diagnostic_impls_IMPL_0038
/* FP:diagnostic_impls.rs-0076 */ impl IntoDiagArg for ast :: FloatTy { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (self . name_str ())) } }