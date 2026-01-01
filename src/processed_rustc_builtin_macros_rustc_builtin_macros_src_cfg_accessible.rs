/* FP:cfg_accessible.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_accessible_USE_0001
/* FP:cfg_accessible.rs-0002 */ use rustc_ast as ast ;
/* FP:cfg_accessible.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_accessible_USE_0002
/* FP:cfg_accessible.rs-0004 */ use rustc_attr_parsing :: validate_attr ;
/* FP:cfg_accessible.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_accessible_USE_0003
/* FP:cfg_accessible.rs-0006 */ use crate :: rustc_expand :: base :: { Annotatable , ExpandResult , ExtCtxt , Indeterminate , MultiItemModifier } ;
/* FP:cfg_accessible.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_accessible_USE_0004
/* FP:cfg_accessible.rs-0008 */ use crate :: rustc_feature :: AttributeTemplate ;
/* FP:cfg_accessible.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_accessible_USE_0005
/* FP:cfg_accessible.rs-0010 */ use crate :: rustc_complete :: { Span , sym } ;
/* FP:cfg_accessible.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_accessible_USE_0006
/* FP:cfg_accessible.rs-0012 */ use crate :: errors ;
/* FP:cfg_accessible.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_accessible_STRUCT_0007
/* FP:cfg_accessible.rs-0014 */ pub (crate) struct Expander ;
/* FP:cfg_accessible.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_accessible_FN_0008
/* FP:cfg_accessible.rs-0016 */ fn validate_input < 'a > (ecx : & ExtCtxt < '_ > , mi : & 'a ast :: MetaItem) -> Option < & 'a ast :: Path > { use errors :: CfgAccessibleInvalid :: * ; match mi . meta_item_list () { None => { } Some ([]) => { ecx . dcx () . emit_err (UnspecifiedPath (mi . span)) ; } Some ([_ , .. , l]) => { ecx . dcx () . emit_err (MultiplePaths (l . span ())) ; } Some ([nmi]) => match nmi . meta_item () { None => { ecx . dcx () . emit_err (LiteralPath (nmi . span ())) ; } Some (mi) => { if ! mi . is_word () { ecx . dcx () . emit_err (HasArguments (mi . span)) ; } return Some (& mi . path) ; } } , } None }
/* FP:cfg_accessible.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_accessible_IMPL_0009
/* FP:cfg_accessible.rs-0018 */ impl MultiItemModifier for Expander { fn expand (& self , ecx : & mut ExtCtxt < '_ > , span : Span , meta_item : & ast :: MetaItem , item : Annotatable , _is_derive_const : bool ,) -> ExpandResult < Vec < Annotatable > , Annotatable > { let template = AttributeTemplate { list : Some (& ["path"]) , .. Default :: default () } ; validate_attr :: check_builtin_meta_item (& ecx . sess . psess , meta_item , ast :: AttrStyle :: Outer , sym :: cfg_accessible , template , true ,) ; let Some (path) = validate_input (ecx , meta_item) else { return ExpandResult :: Ready (Vec :: new ()) ; } ; match ecx . resolver . cfg_accessible (ecx . current_expansion . id , path) { Ok (true) => ExpandResult :: Ready (vec ! [item]) , Ok (false) => ExpandResult :: Ready (Vec :: new ()) , Err (Indeterminate) if ecx . force_mode => { ecx . dcx () . emit_err (errors :: CfgAccessibleIndeterminate { span }) ; ExpandResult :: Ready (vec ! [item]) } Err (Indeterminate) => ExpandResult :: Retry (item) , } } }