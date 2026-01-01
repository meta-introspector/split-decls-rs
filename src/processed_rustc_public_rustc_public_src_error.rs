/* FP:error.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_USE_0001
/* FP:error.rs-0002 */ use std :: fmt :: { Debug , Display , Formatter } ;
/* FP:error.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_USE_0002
/* FP:error.rs-0004 */ use std :: { fmt , io } ;
/* FP:error.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_USE_0003
/* FP:error.rs-0006 */ use crate :: rustc_public_bridge :: bridge ;
/* FP:error.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_MACRO_0004
/* FP:error.rs-0008 */ macro_rules ! error { ($ fmt : literal $ (,) ?) => { Error (format ! ($ fmt)) } ; ($ fmt : literal , $ ($ arg : tt) *) => { Error (format ! ($ fmt , $ ($ arg) *)) } ; }
/* FP:error.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_USE_0005
/* FP:error.rs-0010 */ pub (crate) use error ;
/* FP:error.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_ENUM_0006
/* FP:error.rs-0012 */ # [doc = " An error type used to represent an error that has already been reported by the compiler."] # [derive (Clone , Copy , PartialEq , Eq)] pub enum CompilerError < T > { # [doc = " Compilation failed, either due to normal errors or ICE."] Failed , # [doc = " Compilation was interrupted."] Interrupted (T) , # [doc = " Compilation skipped. This happens when users invoke rustc to retrieve information such as"] # [doc = " --version."] Skipped , }
/* FP:error.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_STRUCT_0007
/* FP:error.rs-0014 */ # [doc = " A generic error to represent an API request that cannot be fulfilled."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Error (pub (crate) String) ;
/* FP:error.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_IMPL_0008
/* FP:error.rs-0016 */ impl bridge :: Error for Error { fn new (msg : String) -> Self { Self (msg) } fn from_internal < T : Debug > (err : T) -> Self { Self (format ! ("{err:?}")) } }
/* FP:error.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_IMPL_0009
/* FP:error.rs-0018 */ impl From < & str > for Error { fn from (value : & str) -> Self { Self (value . into ()) } }
/* FP:error.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_IMPL_0010
/* FP:error.rs-0020 */ impl Display for Error { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Display :: fmt (& self . 0 , f) } }
/* FP:error.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_IMPL_0011
/* FP:error.rs-0022 */ impl < T > Display for CompilerError < T > where T : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { CompilerError :: Failed => write ! (f , "Compilation Failed") , CompilerError :: Interrupted (reason) => write ! (f , "Compilation Interrupted: {reason}") , CompilerError :: Skipped => write ! (f , "Compilation Skipped") , } } }
/* FP:error.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_IMPL_0012
/* FP:error.rs-0024 */ impl < T > Debug for CompilerError < T > where T : Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { CompilerError :: Failed => write ! (f , "Compilation Failed") , CompilerError :: Interrupted (reason) => write ! (f , "Compilation Interrupted: {reason:?}") , CompilerError :: Skipped => write ! (f , "Compilation Skipped") , } } }
/* FP:error.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_IMPL_0013
/* FP:error.rs-0026 */ impl std :: error :: Error for Error { }
/* FP:error.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_IMPL_0014
/* FP:error.rs-0028 */ impl < T > std :: error :: Error for CompilerError < T > where T : Display + Debug { }
/* FP:error.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_error_IMPL_0015
/* FP:error.rs-0030 */ impl From < io :: Error > for Error { fn from (value : io :: Error) -> Self { Error (value . to_string ()) } }