mkuse!{use std :: borrow :: Cow ;}
mkuse!{use std :: fmt :: { self , Display } ;}
mkuse!{use std :: sync :: OnceLock ;}
mkuse!{use rustc_error_messages :: { DiagArgValue , IntoDiagArg } ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable_Generic , PrintAttribute , current_rustc_version , } ;}
mkuse!{use crate :: attrs :: PrintAttribute ;}
mkitem!{mkstruct!{# [derive (Encodable , Decodable , Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub struct RustcVersion { pub major : u16 , pub minor : u16 , pub patch : u16 , }}}
mkitem!{mkimpl!{impl RustcVersion { pub const CURRENT : Self = current_rustc_version ! () ; pub fn current_overridable () -> Self { * CURRENT_OVERRIDABLE . get_or_init (| | { if let Ok (override_var) = std :: env :: var ("RUSTC_OVERRIDE_VERSION_STRING") && let Some (override_) = Self :: parse_str (& override_var) { override_ } else { Self :: CURRENT } }) } fn parse_str (value : & str) -> Option < Self > { let mut components = value . split ('-') . next () . unwrap () . splitn (3 , '.') ; let major = components . next () ? . parse () . ok () ? ; let minor = components . next () ? . parse () . ok () ? ; let patch = components . next () . unwrap_or ("0") . parse () . ok () ? ; Some (RustcVersion { major , minor , patch }) } }}}
mkitem!{static CURRENT_OVERRIDABLE : OnceLock < RustcVersion > = OnceLock :: new () ;}
mkitem!{mkimpl!{impl Display for RustcVersion { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (formatter , "{}.{}.{}" , self . major , self . minor , self . patch) } }}}
mkitem!{mkimpl!{impl IntoDiagArg for RustcVersion { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string ())) } }}}