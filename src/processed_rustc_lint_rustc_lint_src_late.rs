/* FP:late.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0001
/* FP:late.rs-0002 */ use std :: any :: Any ;
/* FP:late.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0002
/* FP:late.rs-0004 */ use std :: cell :: Cell ;
/* FP:late.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0003
/* FP:late.rs-0006 */ use crate :: rustc_data_structures :: stack :: ensure_sufficient_stack ;
/* FP:late.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0004
/* FP:late.rs-0008 */ use crate :: rustc_data_structures :: sync :: join ;
/* FP:late.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0005
/* FP:late.rs-0010 */ use crate :: rustc_complete :: def_id :: { LocalDefId , LocalModDefId } ;
/* FP:late.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0006
/* FP:late.rs-0012 */ use crate :: rustc_complete :: { self as hir , AmbigArg , HirId , intravisit as hir_visit } ;
/* FP:late.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0007
/* FP:late.rs-0014 */ use crate :: rustc_complete :: hir :: nested_filter ;
/* FP:late.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0008
/* FP:late.rs-0016 */ use crate :: rustc_complete :: ty :: { self , TyCtxt } ;
/* FP:late.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0009
/* FP:late.rs-0018 */ use crate :: rustc_complete :: Session ;
/* FP:late.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0010
/* FP:late.rs-0020 */ use crate :: rustc_complete :: lint :: LintPass ;
/* FP:late.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0011
/* FP:late.rs-0022 */ use crate :: rustc_complete :: lint :: builtin :: HardwiredLints ;
/* FP:late.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0012
/* FP:late.rs-0024 */ use crate :: rustc_complete :: Span ;
/* FP:late.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0013
/* FP:late.rs-0026 */ use tracing :: debug ;
/* FP:late.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0014
/* FP:late.rs-0028 */ use crate :: passes :: LateLintPassObject ;
/* FP:late.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_USE_0015
/* FP:late.rs-0030 */ use crate :: { LateContext , LateLintPass , LintId , LintStore } ;
/* FP:late.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_FN_0016
/* FP:late.rs-0032 */ # [doc = " Extract the [`LintStore`] from [`Session`]."] # [doc = ""] # [doc = " This function exists because [`Session::lint_store`] is type-erased."] pub fn unerased_lint_store (sess : & Session) -> & LintStore { let store : & dyn Any = sess . lint_store . as_deref () . unwrap () ; store . downcast_ref () . unwrap () }
/* FP:late.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_MACRO_0017
/* FP:late.rs-0034 */ macro_rules ! lint_callback { ($ cx : expr , $ f : ident , $ ($ args : expr) ,*) => ({ $ cx . pass .$ f (&$ cx . context , $ ($ args) ,*) ; }) }
/* FP:late.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_STRUCT_0018
/* FP:late.rs-0036 */ # [doc = " Implements the AST traversal for late lint passes. `T` provides the"] # [doc = " `check_*` methods."] struct LateContextAndPass < 'tcx , T : LateLintPass < 'tcx > > { context : LateContext < 'tcx > , pass : T , }
/* FP:late.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_IMPL_0019
/* FP:late.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_IMPL_0020
/* FP:late.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_STRUCT_0021
/* FP:late.rs-0042 */ struct RuntimeCombinedLateLintPass < 'a , 'tcx > { passes : & 'a mut [LateLintPassObject < 'tcx >] , }
/* FP:late.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_IMPL_0022
/* FP:late.rs-0044 */ # [allow (rustc :: lint_pass_impl_without_macro)] impl LintPass for RuntimeCombinedLateLintPass < '_ , '_ > { fn name (& self) -> & 'static str { panic ! () } fn get_lints (& self) -> crate :: LintVec { panic ! () } }
/* FP:late.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_MACRO_0023
/* FP:late.rs-0046 */ macro_rules ! impl_late_lint_pass { ([] , [$ ($ (# [$ attr : meta]) * fn $ f : ident ($ ($ param : ident : $ arg : ty) ,*) ;) *]) => { impl <'tcx > LateLintPass <'tcx > for RuntimeCombinedLateLintPass <'_ , 'tcx > { $ (fn $ f (& mut self , context : & LateContext <'tcx >, $ ($ param : $ arg) ,*) { for pass in self . passes . iter_mut () { pass .$ f (context , $ ($ param) ,*) ; } }) * } } ; }
/* FP:late.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_MACRO_0024
/* FP:late.rs-0048 */ crate :: late_lint_methods ! (impl_late_lint_pass , []) ;
/* FP:late.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_FN_0025
/* FP:late.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_FN_0026
/* FP:late.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_FN_0027
/* FP:late.rs-0054 */ fn late_lint_crate < 'tcx > (tcx : TyCtxt < 'tcx >) { let passes : Vec < _ > = unerased_lint_store (tcx . sess) . late_passes . iter () . map (| mk_pass | (mk_pass) (tcx)) . collect () ; if passes . is_empty () { return ; } let context = LateContext { tcx , enclosing_body : None , cached_typeck_results : Cell :: new (None) , param_env : ty :: ParamEnv :: empty () , effective_visibilities : tcx . effective_visibilities (()) , last_node_with_lint_attrs : hir :: CRATE_HIR_ID , generics : None , only_module : false , } ; let lints_that_dont_need_to_run = tcx . lints_that_dont_need_to_run (()) ; let mut filtered_passes : Vec < Box < dyn LateLintPass < 'tcx > > > = passes . into_iter () . filter (| pass | { let lints = (* * pass) . get_lints () ; lints . is_empty () || ! lints . iter () . all (| lint | lints_that_dont_need_to_run . contains (& LintId :: of (lint))) }) . collect () ; filtered_passes . push (Box :: new (HardwiredLints)) ; let pass = RuntimeCombinedLateLintPass { passes : & mut filtered_passes [..] } ; late_lint_crate_inner (tcx , context , pass) ; }
/* FP:late.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_FN_0028
/* FP:late.rs-0056 */ fn late_lint_crate_inner < 'tcx , T : LateLintPass < 'tcx > > (tcx : TyCtxt < 'tcx > , context : LateContext < 'tcx > , pass : T ,) { let mut cx = LateContextAndPass { context , pass } ; cx . with_lint_attrs (hir :: CRATE_HIR_ID , | cx | { lint_callback ! (cx , check_crate ,) ; tcx . hir_walk_toplevel_module (cx) ; lint_callback ! (cx , check_crate_post ,) ; }) }
/* FP:late.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_late_FN_0029