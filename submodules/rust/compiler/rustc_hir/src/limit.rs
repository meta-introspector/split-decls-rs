mkuse!{use std :: fmt ;}
mkuse!{use std :: ops :: { Div , Mul } ;}
mkuse!{use rustc_error_messages :: { DiagArgValue , IntoDiagArg } ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable_Generic } ;}
mkitem!{mkstruct!{# [doc = " New-type wrapper around `usize` for representing limits. Ensures that comparisons against"] # [doc = " limits are consistent throughout the compiler."] # [derive (Clone , Copy , Debug , HashStable_Generic , Encodable , Decodable)] pub struct Limit (pub usize) ;}}
mkitem!{mkimpl!{impl Limit { # [doc = " Create a new limit from a `usize`."] pub fn new (value : usize) -> Self { Limit (value) } # [doc = " Create a new unlimited limit."] pub fn unlimited () -> Self { Limit (usize :: MAX) } # [doc = " Check that `value` is within the limit. Ensures that the same comparisons are used"] # [doc = " throughout the compiler, as mismatches can cause ICEs, see #72540."] # [inline] pub fn value_within_limit (& self , value : usize) -> bool { value <= self . 0 } }}}
mkitem!{mkimpl!{impl From < usize > for Limit { fn from (value : usize) -> Self { Self :: new (value) } }}}
mkitem!{mkimpl!{impl fmt :: Display for Limit { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }}}
mkitem!{mkimpl!{impl Div < usize > for Limit { type Output = Limit ; fn div (self , rhs : usize) -> Self :: Output { Limit :: new (self . 0 / rhs) } }}}
mkitem!{mkimpl!{impl Mul < usize > for Limit { type Output = Limit ; fn mul (self , rhs : usize) -> Self :: Output { Limit :: new (self . 0 * rhs) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for Limit { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_string () . into_diag_arg (& mut None) } }}}