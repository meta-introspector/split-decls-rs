mkuse!{use std :: backtrace :: Backtrace ;}
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use std :: fmt ;}
mkuse!{use std :: num :: ParseIntError ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: process :: ExitStatus ;}
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_ast_pretty :: pprust ;}
mkuse!{use rustc_span :: edition :: Edition ;}
mkuse!{use crate :: { DiagArgValue , IntoDiagArg } ;}
mkitem!{mkstruct!{pub struct DiagArgFromDisplay < 'a > (pub & 'a dyn fmt :: Display) ;}}
mkitem!{mkimpl!{impl IntoDiagArg for DiagArgFromDisplay < '_ > { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . 0 . to_string () . into_diag_arg (path) } }}}
mkitem!{mkimpl!{impl < 'a > From < & 'a dyn fmt :: Display > for DiagArgFromDisplay < 'a > { fn from (t : & 'a dyn fmt :: Display) -> Self { DiagArgFromDisplay (t) } }}}
mkitem!{mkimpl!{impl < 'a , T : fmt :: Display > From < & 'a T > for DiagArgFromDisplay < 'a > { fn from (t : & 'a T) -> Self { DiagArgFromDisplay (t) } }}}
mkitem!{mkimpl!{impl < 'a , T : Clone + IntoDiagArg > IntoDiagArg for & 'a T { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . clone () . into_diag_arg (path) } }}}
mkitem!{# [macro_export] macro_rules ! into_diag_arg_using_display { ($ ($ ty : ty) ,+ $ (,) ?) => { $ (impl $ crate :: IntoDiagArg for $ ty { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> $ crate :: DiagArgValue { self . to_string () . into_diag_arg (path) } }) + } }}
mkitem!{macro_rules ! into_diag_arg_for_number { ($ ($ ty : ty) ,+ $ (,) ?) => { $ (impl $ crate :: IntoDiagArg for $ ty { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> $ crate :: DiagArgValue { # [allow (irrefutable_let_patterns)] if let Ok (n) = TryInto ::< i32 >:: try_into (self) { $ crate :: DiagArgValue :: Number (n) } else { self . to_string () . into_diag_arg (path) } } }) + } }}
mkitem!{into_diag_arg_using_display ! (ast :: ParamKindOrd , std :: io :: Error , Box < dyn std :: error :: Error >, std :: num :: NonZero < u32 >, Edition , rustc_span :: Ident , rustc_span :: MacroRulesNormalizedIdent , ParseIntError , ExitStatus ,) ;}
mkitem!{into_diag_arg_for_number ! (i8 , u8 , i16 , u16 , i32 , u32 , i64 , u64 , i128 , u128 , isize , usize) ;}
mkitem!{mkimpl!{impl IntoDiagArg for bool { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { if self { DiagArgValue :: Str (Cow :: Borrowed ("true")) } else { DiagArgValue :: Str (Cow :: Borrowed ("false")) } } }}}
mkitem!{mkimpl!{impl IntoDiagArg for char { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (format ! ("{self:?}"))) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for Vec < char > { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: StrListSepByAnd (self . into_iter () . map (| c | Cow :: Owned (format ! ("{c:?}"))) . collect () ,) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for rustc_span :: Symbol { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_ident_string () . into_diag_arg (path) } }}}
mkitem!{mkimpl!{impl < 'a > IntoDiagArg for & 'a str { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_string () . into_diag_arg (path) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for String { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self)) } }}}
mkitem!{mkimpl!{impl < 'a > IntoDiagArg for Cow < 'a , str > { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . into_owned ())) } }}}
mkitem!{mkimpl!{impl < 'a > IntoDiagArg for & 'a Path { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . display () . to_string ())) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for PathBuf { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . display () . to_string ())) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for ast :: Expr { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (pprust :: expr_to_string (& self))) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for ast :: Path { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (pprust :: path_to_string (& self))) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for ast :: token :: Token { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (pprust :: token_to_string (& self)) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for ast :: token :: TokenKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (pprust :: token_kind_to_string (& self)) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for std :: ffi :: CString { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string_lossy () . into_owned ())) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for rustc_data_structures :: small_c_str :: SmallCStr { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string_lossy () . into_owned ())) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for ast :: Visibility { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { let s = pprust :: vis_to_string (& self) ; let s = s . trim_end () . to_string () ; DiagArgValue :: Str (Cow :: Owned (s)) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for Backtrace { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: from (self . to_string ())) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for ast :: util :: parser :: ExprPrecedence { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Number (self as i32) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for ast :: FloatTy { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (self . name_str ())) } }}}