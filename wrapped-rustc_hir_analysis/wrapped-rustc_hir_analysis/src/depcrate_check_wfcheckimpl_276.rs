// Generated macro for impl_276 (impl)
macro_rules! Depcrate_check_wfcheckimpl_276 {
() => {
// Module: crate::check::wfcheck
// Provides: {"impl_276"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for CollectUsageSpans < '_ > { type Result = () ; fn visit_generics (& mut self , _g : & 'tcx rustc_hir :: Generics < 'tcx >) -> Self :: Result { } fn visit_ty (& mut self , t : & 'tcx hir :: Ty < 'tcx , AmbigArg >) -> Self :: Result { if let hir :: TyKind :: Path (hir :: QPath :: Resolved (None , qpath)) = t . kind { if let Res :: Def (DefKind :: TyParam , def_id) = qpath . res && def_id == self . param_def_id { self . spans . push (t . span) ; return ; } else if let Res :: SelfTyAlias { .. } = qpath . res { self . spans . push (t . span) ; return ; } } intravisit :: walk_ty (self , t) ; } }
};
}
