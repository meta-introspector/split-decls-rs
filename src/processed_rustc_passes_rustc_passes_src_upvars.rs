/* FP:upvars.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_USE_0001
/* FP:upvars.rs-0002 */ use crate :: rustc_data_structures :: fx :: { FxHashSet , FxIndexMap } ;
/* FP:upvars.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_USE_0002
/* FP:upvars.rs-0004 */ use rustc_hir as hir ;
/* FP:upvars.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_USE_0003
/* FP:upvars.rs-0006 */ use crate :: rustc_complete :: def :: Res ;
/* FP:upvars.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_USE_0004
/* FP:upvars.rs-0008 */ use crate :: rustc_complete :: intravisit :: { self , Visitor } ;
/* FP:upvars.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_USE_0005
/* FP:upvars.rs-0010 */ use crate :: rustc_complete :: { self , HirId } ;
/* FP:upvars.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_USE_0006
/* FP:upvars.rs-0012 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:upvars.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_USE_0007
/* FP:upvars.rs-0014 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:upvars.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_USE_0008
/* FP:upvars.rs-0016 */ use crate :: rustc_complete :: Span ;
/* FP:upvars.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_FN_0009
/* FP:upvars.rs-0018 */ pub (crate) fn provide (providers : & mut Providers) { providers . upvars_mentioned = | tcx , def_id | { if ! tcx . is_closure_like (def_id) { return None ; } let local_def_id = def_id . expect_local () ; let body = tcx . hir_maybe_body_owned_by (local_def_id) ? ; let mut local_collector = LocalCollector :: default () ; local_collector . visit_body (& body) ; let mut capture_collector = CaptureCollector { tcx , locals : & local_collector . locals , upvars : FxIndexMap :: default () , } ; capture_collector . visit_body (& body) ; if ! capture_collector . upvars . is_empty () { Some (tcx . arena . alloc (capture_collector . upvars)) } else { None } } ; }
/* FP:upvars.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_STRUCT_0010
/* FP:upvars.rs-0020 */ # [derive (Default)] struct LocalCollector { locals : FxHashSet < HirId > , }
/* FP:upvars.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_IMPL_0011
/* FP:upvars.rs-0022 */ impl < 'tcx > Visitor < 'tcx > for LocalCollector { fn visit_pat (& mut self , pat : & 'tcx hir :: Pat < 'tcx >) { if let hir :: PatKind :: Binding (_ , hir_id , ..) = pat . kind { self . locals . insert (hir_id) ; } intravisit :: walk_pat (self , pat) ; } }
/* FP:upvars.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_STRUCT_0012
/* FP:upvars.rs-0024 */ struct CaptureCollector < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , locals : & 'a FxHashSet < HirId > , upvars : FxIndexMap < HirId , hir :: Upvar > , }
/* FP:upvars.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_IMPL_0013
/* FP:upvars.rs-0026 */ impl CaptureCollector < '_ , '_ > { fn visit_local_use (& mut self , var_id : HirId , span : Span) { if ! self . locals . contains (& var_id) { self . upvars . entry (var_id) . or_insert (hir :: Upvar { span }) ; } } }
/* FP:upvars.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_upvars_IMPL_0014
/* FP:upvars.rs-0028 */ impl < 'tcx > Visitor < 'tcx > for CaptureCollector < '_ , 'tcx > { fn visit_path (& mut self , path : & hir :: Path < 'tcx > , _ : HirId) { if let Res :: Local (var_id) = path . res { self . visit_local_use (var_id , path . span) ; } intravisit :: walk_path (self , path) ; } fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Closure (closure) = expr . kind && let Some (upvars) = self . tcx . upvars_mentioned (closure . def_id) { for (& var_id , upvar) in upvars { self . visit_local_use (var_id , upvar . span) ; } } intravisit :: walk_expr (self , expr) ; } }