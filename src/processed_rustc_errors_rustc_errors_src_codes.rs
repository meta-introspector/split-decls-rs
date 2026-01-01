/* FP:codes.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_codes_USE_0001
/* FP:codes.rs-0002 */ use std :: fmt ;
/* FP:codes.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_codes_MACRO_0002
/* FP:codes.rs-0004 */ crate :: rustc_index :: newtype_index ! { # [max = 9999] # [orderable] # [encodable] # [debug_format = "ErrCode({})"] pub struct ErrCode { } }
/* FP:codes.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_codes_IMPL_0003
/* FP:codes.rs-0006 */ impl fmt :: Display for ErrCode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "E{:04}" , self . as_u32 ()) } }
/* FP:codes.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_codes_MACRO_0004
/* FP:codes.rs-0008 */ crate :: rustc_error_messages :: into_diag_arg_using_display ! (ErrCode) ;
/* FP:codes.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_codes_MACRO_0005
/* FP:codes.rs-0010 */ macro_rules ! define_error_code_constants_and_diagnostics_table { ($ ($ name : ident : $ num : literal ,) *) => ($ (pub const $ name : $ crate :: ErrCode = $ crate :: ErrCode :: from_u32 ($ num) ;) * pub static DIAGNOSTICS : & [($ crate :: ErrCode , & str)] = & [$ (($ name , include_str ! (concat ! ("../../rustc_error_codes/src/error_codes/" , stringify ! ($ name) , ".md"))) ,) *] ;) }
/* FP:codes.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_codes_MACRO_0006
/* FP:codes.rs-0012 */ rustc_error_codes :: error_codes ! (define_error_code_constants_and_diagnostics_table) ;