mkuse!{use std :: fmt ;}
mkitem!{rustc_index :: newtype_index ! { # [max = 9999] # [orderable] # [encodable] # [debug_format = "ErrCode({})"] pub struct ErrCode { } }}
mkitem!{mkimpl!{impl fmt :: Display for ErrCode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "E{:04}" , self . as_u32 ()) } }}}
mkitem!{rustc_error_messages :: into_diag_arg_using_display ! (ErrCode) ;}
mkitem!{macro_rules ! define_error_code_constants_and_diagnostics_table { ($ ($ name : ident : $ num : literal ,) *) => ($ (pub const $ name : $ crate :: ErrCode = $ crate :: ErrCode :: from_u32 ($ num) ;) * pub static DIAGNOSTICS : & [($ crate :: ErrCode , & str)] = & [$ (($ name , include_str ! (concat ! ("../../rustc_error_codes/src/error_codes/" , stringify ! ($ name) , ".md"))) ,) *] ;) }}
mkitem!{rustc_error_codes :: error_codes ! (define_error_code_constants_and_diagnostics_table) ;}