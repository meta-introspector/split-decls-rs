// Generated macro for impl_1599 (impl)
macro_rules! Depcrate_disallowed_typesimpl_1599 {
() => {
// Module: crate::disallowed_types
// Provides: {"impl_1599"}
// Dependencies: {}
impl DisallowedTypes { pub fn new (tcx : TyCtxt < '_ > , conf : & 'static Conf) -> Self { let (def_ids , prim_tys) = create_disallowed_map (tcx , & conf . disallowed_types , PathNS :: Type , def_kind_predicate , "type" , true ,) ; Self { def_ids , prim_tys } } fn check_res_emit (& self , cx : & LateContext < '_ > , res : & Res , span : Span) { let (path , disallowed_path) = match res { Res :: Def (_ , did) if let Some (& x) = self . def_ids . get (did) => x , Res :: PrimTy (prim) if let Some (& x) = self . prim_tys . get (prim) => x , _ => return , } ; span_lint_and_then (cx , DISALLOWED_TYPES , span , format ! ("use of a disallowed type `{path}`") , disallowed_path . diag_amendment (span) ,) ; } }
};
}
