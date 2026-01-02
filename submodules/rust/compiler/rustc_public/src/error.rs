mkuse!{use std :: fmt :: { Debug , Display , Formatter } ;}
mkuse!{use std :: { fmt , io } ;}
mkuse!{use rustc_public_bridge :: bridge ;}
mkitem!{macro_rules ! error { ($ fmt : literal $ (,) ?) => { Error (format ! ($ fmt)) } ; ($ fmt : literal , $ ($ arg : tt) *) => { Error (format ! ($ fmt , $ ($ arg) *)) } ; }}
mkuse!{pub (crate) use error ;}
mkitem!{mkenum!{# [doc = " An error type used to represent an error that has already been reported by the compiler."] # [derive (Clone , Copy , PartialEq , Eq)] pub enum CompilerError < T > { # [doc = " Compilation failed, either due to normal errors or ICE."] Failed , # [doc = " Compilation was interrupted."] Interrupted (T) , # [doc = " Compilation skipped. This happens when users invoke rustc to retrieve information such as"] # [doc = " --version."] Skipped , }}}
mkitem!{mkstruct!{# [doc = " A generic error to represent an API request that cannot be fulfilled."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Error (pub (crate) String) ;}}
mkitem!{mkimpl!{impl bridge :: Error for Error { fn new (msg : String) -> Self { Self (msg) } fn from_internal < T : Debug > (err : T) -> Self { Self (format ! ("{err:?}")) } }}}
mkitem!{mkimpl!{impl From < & str > for Error { fn from (value : & str) -> Self { Self (value . into ()) } }}}
mkitem!{mkimpl!{impl Display for Error { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Display :: fmt (& self . 0 , f) } }}}
mkitem!{mkimpl!{impl < T > Display for CompilerError < T > where T : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { CompilerError :: Failed => write ! (f , "Compilation Failed") , CompilerError :: Interrupted (reason) => write ! (f , "Compilation Interrupted: {reason}") , CompilerError :: Skipped => write ! (f , "Compilation Skipped") , } } }}}
mkitem!{mkimpl!{impl < T > Debug for CompilerError < T > where T : Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { CompilerError :: Failed => write ! (f , "Compilation Failed") , CompilerError :: Interrupted (reason) => write ! (f , "Compilation Interrupted: {reason:?}") , CompilerError :: Skipped => write ! (f , "Compilation Skipped") , } } }}}
mkitem!{mkimpl!{impl std :: error :: Error for Error { }}}
mkitem!{mkimpl!{impl < T > std :: error :: Error for CompilerError < T > where T : Display + Debug { }}}
mkitem!{mkimpl!{impl From < io :: Error > for Error { fn from (value : io :: Error) -> Self { Error (value . to_string ()) } }}}