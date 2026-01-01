/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_USE_0001
/* FP:mod.rs-0002 */ use crate :: rustc_abi :: { FieldIdx , VariantIdx } ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_USE_0002
/* FP:mod.rs-0004 */ use crate :: rustc_complete :: ty :: { self , Ty , TyCtxt } ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_USE_0003
/* FP:mod.rs-0006 */ use crate :: rustc_complete :: { bug , mir } ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_USE_0004
/* FP:mod.rs-0008 */ use crate :: rustc_complete :: DUMMY_SP ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_USE_0005
/* FP:mod.rs-0010 */ use tracing :: instrument ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_USE_0006
/* FP:mod.rs-0012 */ use crate :: interpret :: InterpCx ;
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_MOD_0007
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_MOD_0008
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_MOD_0009
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_MOD_0010
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_MOD_0011
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_MOD_0012
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_USE_0013
/* FP:mod.rs-0026 */ pub use self :: dummy_machine :: * ;
/* FP:mod.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_USE_0014
/* FP:mod.rs-0028 */ pub use self :: error :: * ;
/* FP:mod.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_USE_0015
/* FP:mod.rs-0030 */ pub use self :: eval_queries :: * ;
/* FP:mod.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_USE_0016
/* FP:mod.rs-0032 */ pub use self :: fn_queries :: * ;
/* FP:mod.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_USE_0017
/* FP:mod.rs-0034 */ pub use self :: machine :: * ;
/* FP:mod.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_USE_0018
/* FP:mod.rs-0036 */ pub (crate) use self :: valtrees :: { eval_to_valtree , valtree_to_const_value } ;
/* FP:mod.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_CONST_0019
/* FP:mod.rs-0038 */ const VALTREE_MAX_NODES : usize = 100000 ;
/* FP:mod.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_FN_0020
/* FP:mod.rs-0040 */ # [instrument (skip (tcx) , level = "debug")] pub (crate) fn try_destructure_mir_constant_for_user_output < 'tcx > (tcx : TyCtxt < 'tcx > , val : mir :: ConstValue , ty : Ty < 'tcx > ,) -> Option < mir :: DestructuredConstant < 'tcx > > { let typing_env = ty :: TypingEnv :: fully_monomorphized () ; let (ecx , op) = mk_eval_cx_for_const_val (tcx . at (crate :: rustc_span :: DUMMY_SP) , typing_env , val , ty) ? ; let (field_count , variant , down) = match ty . kind () { ty :: Array (_ , len) => (len . try_to_target_usize (tcx) ? as usize , None , op) , ty :: Adt (def , _) if def . variants () . is_empty () => { return None ; } ty :: Adt (def , _) => { let variant = ecx . read_discriminant (& op) . discard_err () ? ; let down = ecx . project_downcast (& op , variant) . discard_err () ? ; (def . variants () [variant] . fields . len () , Some (variant) , down) } ty :: Tuple (args) => (args . len () , None , op) , _ => bug ! ("cannot destructure mir constant {:?}" , val) , } ; let fields_iter = (0 .. field_count) . map (| i | { let field_op = ecx . project_field (& down , FieldIdx :: from_usize (i)) . discard_err () ? ; let val = op_to_const (& ecx , & field_op , true) ; Some ((val , field_op . layout . ty)) }) . collect :: < Option < Vec < _ > > > () ? ; let fields = tcx . arena . alloc_from_iter (fields_iter) ; Some (mir :: DestructuredConstant { variant , fields }) }
/* FP:mod.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_const_eval_mod_FN_0021
/* FP:mod.rs-0042 */ # [doc = " Computes the tag (if any) for a given type and variant."] # [instrument (skip (tcx) , level = "debug")] pub fn tag_for_variant_provider < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , (Ty < 'tcx > , VariantIdx) > ,) -> Option < ty :: ScalarInt > { let (ty , variant_index) = key . value ; assert ! (ty . is_enum ()) ; let ecx = InterpCx :: new (tcx , DUMMY_SP , key . typing_env , crate :: const_eval :: DummyMachine) ; let layout = ecx . layout_of (ty) . unwrap () ; ecx . tag_for_variant (layout , variant_index) . unwrap () . map (| (tag , _tag_field) | tag) }