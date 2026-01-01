/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_errors_USE_0001
/* FP:errors.rs-0002 */ use std :: fmt ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: { Diag , DiagCtxtHandle , Diagnostic , EmissionGuarantee , Level } ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_errors_USE_0003
/* FP:errors.rs-0006 */ use crate :: rustc_complete :: Span ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_errors_STRUCT_0004
/* FP:errors.rs-0008 */ pub struct TestOutput { pub span : Span , pub kind : Kind , pub content : String , }
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_errors_IMPL_0005
/* FP:errors.rs-0010 */ impl < G : EmissionGuarantee > Diagnostic < '_ , G > for TestOutput { fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let TestOutput { span , kind , content } = self ; # [allow (rustc :: untranslatable_diagnostic)] Diag :: new (dcx , level , format ! ("{kind}({content})")) . with_span (span) } }
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_errors_ENUM_0006
/* FP:errors.rs-0012 */ pub enum Kind { SymbolName , Demangling , DemanglingAlt , DefPath , }
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_errors_IMPL_0007
/* FP:errors.rs-0014 */ impl fmt :: Display for Kind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Kind :: SymbolName => write ! (f , "symbol-name") , Kind :: Demangling => write ! (f , "demangling") , Kind :: DemanglingAlt => write ! (f , "demangling-alt") , Kind :: DefPath => write ! (f , "def-path") , } } }