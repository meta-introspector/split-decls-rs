/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_USE_0001
/* FP:mod.rs-0002 */ use rustc_ast as ast ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_USE_0002
/* FP:mod.rs-0004 */ use crate :: rustc_complete :: { GenericArg , MetaItem } ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_USE_0003
/* FP:mod.rs-0006 */ use crate :: rustc_expand :: base :: { Annotatable , ExpandResult , ExtCtxt , MultiItemModifier } ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_USE_0004
/* FP:mod.rs-0008 */ use crate :: rustc_complete :: { Span , Symbol , sym } ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_USE_0005
/* FP:mod.rs-0010 */ use thin_vec :: { ThinVec , thin_vec } ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_OTHER_0006
/* FP:mod.rs-0012 */ macro path_local ($ x : ident) { generic :: ty :: Path :: new_local (sym ::$ x) }
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_OTHER_0007
/* FP:mod.rs-0014 */ macro pathvec_std ($ ($ rest : ident) ::+) { { vec ! [$ (sym ::$ rest) ,+] } }
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_OTHER_0008
/* FP:mod.rs-0016 */ macro path_std ($ ($ x : tt) *) { generic :: ty :: Path :: new (pathvec_std ! ($ ($ x) *)) }
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_MOD_0009
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_MOD_0010
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_MOD_0011
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_MOD_0012
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_MOD_0013
/* FP:mod.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_MOD_0014
/* FP:mod.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_MOD_0015
/* FP:mod.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_MOD_0016
/* FP:mod.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_MOD_0017
/* FP:mod.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_MOD_0018
/* FP:mod.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_MOD_0019
/* FP:mod.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_MOD_0020
/* FP:mod.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_TYPE_0021
/* FP:mod.rs-0042 */ pub (crate) type BuiltinDeriveFn = fn (& ExtCtxt < '_ > , Span , & MetaItem , & Annotatable , & mut dyn FnMut (Annotatable) , bool) ;
/* FP:mod.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_STRUCT_0022
/* FP:mod.rs-0044 */ pub (crate) struct BuiltinDerive (pub (crate) BuiltinDeriveFn) ;
/* FP:mod.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_IMPL_0023
/* FP:mod.rs-0046 */ impl MultiItemModifier for BuiltinDerive { fn expand (& self , ecx : & mut ExtCtxt < '_ > , span : Span , meta_item : & MetaItem , item : Annotatable , is_derive_const : bool ,) -> ExpandResult < Vec < Annotatable > , Annotatable > { let span = ecx . with_def_site_ctxt (span) ; let mut items = Vec :: new () ; match item { Annotatable :: Stmt (stmt) => { if let ast :: StmtKind :: Item (item) = stmt . kind { (self . 0) (ecx , span , meta_item , & Annotatable :: Item (item) , & mut | a | { items . push (Annotatable :: Stmt (Box :: new (ast :: Stmt { id : ast :: DUMMY_NODE_ID , kind : ast :: StmtKind :: Item (a . expect_item ()) , span , }))) ; } , is_derive_const ,) ; } else { unreachable ! ("should have already errored on non-item statement") } } _ => { (self . 0) (ecx , span , meta_item , & item , & mut | a | items . push (a) , is_derive_const) ; } } ExpandResult :: Ready (items) } }
/* FP:mod.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_FN_0024
/* FP:mod.rs-0048 */ # [doc = " Constructs an expression that calls an intrinsic"] fn call_intrinsic (cx : & ExtCtxt < '_ > , span : Span , intrinsic : Symbol , args : ThinVec < Box < ast :: Expr > > ,) -> Box < ast :: Expr > { let span = cx . with_def_site_ctxt (span) ; let path = cx . std_path (& [sym :: intrinsics , intrinsic]) ; cx . expr_call_global (span , path , args) }
/* FP:mod.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_FN_0025
/* FP:mod.rs-0050 */ # [doc = " Constructs an expression that calls the `unreachable` intrinsic."] fn call_unreachable (cx : & ExtCtxt < '_ > , span : Span) -> Box < ast :: Expr > { let span = cx . with_def_site_ctxt (span) ; let path = cx . std_path (& [sym :: intrinsics , sym :: unreachable]) ; let call = cx . expr_call_global (span , path , ThinVec :: new ()) ; cx . expr_block (Box :: new (ast :: Block { stmts : thin_vec ! [cx . stmt_expr (call)] , id : ast :: DUMMY_NODE_ID , rules : ast :: BlockCheckMode :: Unsafe (ast :: CompilerGenerated) , span , tokens : None , })) }
/* FP:mod.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_mod_FN_0026
/* FP:mod.rs-0052 */ fn assert_ty_bounds (cx : & ExtCtxt < '_ > , stmts : & mut ThinVec < ast :: Stmt > , ty : Box < ast :: Ty > , span : Span , assert_path : & [Symbol] ,) { let span = cx . with_def_site_ctxt (span) ; let assert_path = cx . path_all (span , true , cx . std_path (assert_path) , vec ! [GenericArg :: Type (ty)]) ; stmts . push (cx . stmt_let_type_only (span , cx . ty_path (assert_path))) ; }