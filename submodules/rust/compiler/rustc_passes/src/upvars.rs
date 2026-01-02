mkuse!{use rustc_data_structures :: fx :: { FxHashSet , FxIndexMap } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: Res ;}
mkuse!{use rustc_hir :: intravisit :: { self , Visitor } ;}
mkuse!{use rustc_hir :: { self , HirId } ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_span :: Span ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { providers . upvars_mentioned = | tcx , def_id | { if ! tcx . is_closure_like (def_id) { return None ; } let local_def_id = def_id . expect_local () ; let body = tcx . hir_maybe_body_owned_by (local_def_id) ? ; let mut local_collector = LocalCollector :: default () ; local_collector . visit_body (& body) ; let mut capture_collector = CaptureCollector { tcx , locals : & local_collector . locals , upvars : FxIndexMap :: default () , } ; capture_collector . visit_body (& body) ; if ! capture_collector . upvars . is_empty () { Some (tcx . arena . alloc (capture_collector . upvars)) } else { None } } ; }
}
mkitem!{mkstruct!{# [derive (Default)] struct LocalCollector { locals : FxHashSet < HirId > , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for LocalCollector { fn visit_pat (& mut self , pat : & 'tcx hir :: Pat < 'tcx >) { if let hir :: PatKind :: Binding (_ , hir_id , ..) = pat . kind { self . locals . insert (hir_id) ; } intravisit :: walk_pat (self , pat) ; } }}}
mkitem!{mkstruct!{struct CaptureCollector < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , locals : & 'a FxHashSet < HirId > , upvars : FxIndexMap < HirId , hir :: Upvar > , }}}
mkitem!{mkimpl!{impl CaptureCollector < '_ , '_ > { fn visit_local_use (& mut self , var_id : HirId , span : Span) { if ! self . locals . contains (& var_id) { self . upvars . entry (var_id) . or_insert (hir :: Upvar { span }) ; } } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for CaptureCollector < '_ , 'tcx > { fn visit_path (& mut self , path : & hir :: Path < 'tcx > , _ : HirId) { if let Res :: Local (var_id) = path . res { self . visit_local_use (var_id , path . span) ; } intravisit :: walk_path (self , path) ; } fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Closure (closure) = expr . kind && let Some (upvars) = self . tcx . upvars_mentioned (closure . def_id) { for (& var_id , upvar) in upvars { self . visit_local_use (var_id , upvar . span) ; } } intravisit :: walk_expr (self , expr) ; } }}}