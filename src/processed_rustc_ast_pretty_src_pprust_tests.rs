/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_tests_USE_0001
/* FP:tests.rs-0002 */ use rustc_ast as ast ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_tests_USE_0002
/* FP:tests.rs-0004 */ use crate :: rustc_complete :: { DUMMY_SP , Ident , create_default_session_globals_then } ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_tests_USE_0003
/* FP:tests.rs-0006 */ use thin_vec :: ThinVec ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_tests_USE_0004
/* FP:tests.rs-0008 */ use super :: * ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_tests_FN_0005
/* FP:tests.rs-0010 */ fn fun_to_string (decl : & ast :: FnDecl , header : ast :: FnHeader , ident : Ident , generics : & ast :: Generics ,) -> String { to_string (| s | { let (cb , ib) = s . head ("") ; s . print_fn (decl , header , Some (ident) , generics) ; s . end (ib) ; s . end (cb) ; }) }
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_tests_FN_0006
/* FP:tests.rs-0012 */ fn variant_to_string (var : & ast :: Variant) -> String { to_string (| s | s . print_variant (var)) }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_tests_FN_0007
/* FP:tests.rs-0014 */ # [test] fn test_fun_to_string () { create_default_session_globals_then (| | { let abba_ident = Ident :: from_str ("abba") ; let decl = ast :: FnDecl { inputs : ThinVec :: new () , output : ast :: FnRetTy :: Default (DUMMY_SP) } ; let generics = ast :: Generics :: default () ; assert_eq ! (fun_to_string (& decl , ast :: FnHeader :: default () , abba_ident , & generics) , "fn abba()") ; }) }
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_tests_FN_0008
/* FP:tests.rs-0016 */ # [test] fn test_variant_to_string () { create_default_session_globals_then (| | { let ident = Ident :: from_str ("principal_skinner") ; let var = ast :: Variant { ident , vis : ast :: Visibility { span : DUMMY_SP , kind : ast :: VisibilityKind :: Inherited , tokens : None , } , attrs : ast :: AttrVec :: new () , id : ast :: DUMMY_NODE_ID , data : ast :: VariantData :: Unit (ast :: DUMMY_NODE_ID) , disr_expr : None , span : DUMMY_SP , is_placeholder : false , } ; let varstr = variant_to_string (& var) ; assert_eq ! (varstr , "principal_skinner") ; }) }