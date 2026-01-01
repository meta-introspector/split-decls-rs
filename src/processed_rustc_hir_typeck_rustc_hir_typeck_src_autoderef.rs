/* FP:autoderef.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_typeck_src_autoderef_USE_0001
/* FP:autoderef.rs-0002 */ use std :: iter ;
/* FP:autoderef.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_typeck_src_autoderef_USE_0002
/* FP:autoderef.rs-0004 */ use itertools :: Itertools ;
/* FP:autoderef.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_typeck_src_autoderef_USE_0003
/* FP:autoderef.rs-0006 */ use crate :: rustc_hir_analysis :: autoderef :: { Autoderef , AutoderefKind } ;
/* FP:autoderef.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_typeck_src_autoderef_USE_0004
/* FP:autoderef.rs-0008 */ use crate :: rustc_infer :: infer :: InferOk ;
/* FP:autoderef.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_typeck_src_autoderef_USE_0005
/* FP:autoderef.rs-0010 */ use crate :: rustc_infer :: traits :: PredicateObligations ;
/* FP:autoderef.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_typeck_src_autoderef_USE_0006
/* FP:autoderef.rs-0012 */ use crate :: rustc_complete :: ty :: adjustment :: { Adjust , Adjustment , OverloadedDeref } ;
/* FP:autoderef.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_typeck_src_autoderef_USE_0007
/* FP:autoderef.rs-0014 */ use crate :: rustc_complete :: ty :: { self , Ty } ;
/* FP:autoderef.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_typeck_src_autoderef_USE_0008
/* FP:autoderef.rs-0016 */ use crate :: rustc_complete :: Span ;
/* FP:autoderef.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_typeck_src_autoderef_USE_0009
/* FP:autoderef.rs-0018 */ use super :: method :: MethodCallee ;
/* FP:autoderef.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_typeck_src_autoderef_USE_0010
/* FP:autoderef.rs-0020 */ use super :: { FnCtxt , PlaceOp } ;
/* FP:autoderef.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_typeck_src_autoderef_IMPL_0011
/* FP:autoderef.rs-0022 */ impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { pub (crate) fn autoderef (& 'a self , span : Span , base_ty : Ty < 'tcx >) -> Autoderef < 'a , 'tcx > { Autoderef :: new (self , self . param_env , self . body_id , span , base_ty) } pub (crate) fn try_overloaded_deref (& self , span : Span , base_ty : Ty < 'tcx > ,) -> Option < InferOk < 'tcx , MethodCallee < 'tcx > > > { self . try_overloaded_place_op (span , base_ty , None , PlaceOp :: Deref) } # [doc = " Returns the adjustment steps."] pub (crate) fn adjust_steps (& self , autoderef : & Autoderef < 'a , 'tcx >) -> Vec < Adjustment < 'tcx > > { self . register_infer_ok_obligations (self . adjust_steps_as_infer_ok (autoderef)) } pub (crate) fn adjust_steps_as_infer_ok (& self , autoderef : & Autoderef < 'a , 'tcx > ,) -> InferOk < 'tcx , Vec < Adjustment < 'tcx > > > { let steps = autoderef . steps () ; if steps . is_empty () { return InferOk { obligations : PredicateObligations :: new () , value : vec ! [] } ; } let mut obligations = PredicateObligations :: new () ; let targets = steps . iter () . skip (1) . map (| & (ty , _) | ty) . chain (iter :: once (autoderef . final_ty ())) ; let steps : Vec < _ > = steps . iter () . map (| & (source , kind) | { if let AutoderefKind :: Overloaded = kind { self . try_overloaded_deref (autoderef . span () , source) . and_then (| InferOk { value : method , obligations : o } | { obligations . extend (o) ; if let ty :: Ref (_ , _ , mutbl) = * method . sig . output () . kind () { Some (OverloadedDeref { mutbl , span : autoderef . span () }) } else { None } } ,) } else { None } }) . zip_eq (targets) . map (| (autoderef , target) | Adjustment { kind : Adjust :: Deref (autoderef) , target }) . collect () ; InferOk { obligations , value : steps } } }