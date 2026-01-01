/* FP:eq.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_cmp_eq_USE_0001
/* FP:eq.rs-0002 */ use crate :: rustc_complete :: { self as ast , MetaItem } ;
/* FP:eq.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_cmp_eq_USE_0002
/* FP:eq.rs-0004 */ use crate :: rustc_data_structures :: fx :: FxHashSet ;
/* FP:eq.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_cmp_eq_USE_0003
/* FP:eq.rs-0006 */ use crate :: rustc_expand :: base :: { Annotatable , ExtCtxt } ;
/* FP:eq.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_cmp_eq_USE_0004
/* FP:eq.rs-0008 */ use crate :: rustc_complete :: { Span , sym } ;
/* FP:eq.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_cmp_eq_USE_0005
/* FP:eq.rs-0010 */ use thin_vec :: { ThinVec , thin_vec } ;
/* FP:eq.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_cmp_eq_USE_0006
/* FP:eq.rs-0012 */ use crate :: deriving :: generic :: ty :: * ;
/* FP:eq.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_cmp_eq_USE_0007
/* FP:eq.rs-0014 */ use crate :: deriving :: generic :: * ;
/* FP:eq.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_cmp_eq_USE_0008
/* FP:eq.rs-0016 */ use crate :: deriving :: path_std ;
/* FP:eq.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_cmp_eq_FN_0009
/* FP:eq.rs-0018 */ pub (crate) fn expand_deriving_eq (cx : & ExtCtxt < '_ > , span : Span , mitem : & MetaItem , item : & Annotatable , push : & mut dyn FnMut (Annotatable) , is_const : bool ,) { let span = cx . with_def_site_ctxt (span) ; let trait_def = TraitDef { span , path : path_std ! (cmp :: Eq) , skip_path_as_bound : false , needs_copy_as_bound_if_packed : true , additional_bounds : Vec :: new () , supports_unions : true , methods : vec ! [MethodDef { name : sym :: assert_receiver_is_total_eq , generics : Bounds :: empty () , explicit_self : true , nonself_args : vec ! [] , ret_ty : Unit , attributes : thin_vec ! [cx . attr_word (sym :: inline , span) , cx . attr_nested_word (sym :: doc , sym :: hidden , span) , cx . attr_nested_word (sym :: coverage , sym :: off , span)] , fieldless_variants_strategy : FieldlessVariantsStrategy :: Unify , combine_substructure : combine_substructure (Box :: new (| a , b , c | { cs_total_eq_assert (a , b , c) })) , }] , associated_types : Vec :: new () , is_const , is_staged_api_crate : cx . ecfg . features . staged_api () , } ; trait_def . expand_ext (cx , mitem , item , push , true) }
/* FP:eq.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_deriving_cmp_eq_FN_0010
/* FP:eq.rs-0020 */ fn cs_total_eq_assert (cx : & ExtCtxt < '_ > , trait_span : Span , substr : & Substructure < '_ > ,) -> BlockOrExpr { let mut stmts = ThinVec :: new () ; let mut seen_type_names = FxHashSet :: default () ; let mut process_variant = | variant : & ast :: VariantData | { for field in variant . fields () { if let Some (name) = field . ty . kind . is_simple_path () && ! seen_type_names . insert (name) { } else { super :: assert_ty_bounds (cx , & mut stmts , field . ty . clone () , field . span , & [sym :: cmp , sym :: AssertParamIsEq] ,) ; } } } ; match * substr . fields { StaticStruct (vdata , ..) => { process_variant (vdata) ; } StaticEnum (enum_def , ..) => { for variant in & enum_def . variants { process_variant (& variant . data) ; } } _ => cx . dcx () . span_bug (trait_span , "unexpected substructure in `derive(Eq)`") , } BlockOrExpr :: new_stmts (stmts) }