// Generated macro for type_for_const (function)
macro_rules! Depcrate_lowertype_for_const {
() => {
// Module: crate::lower
// Provides: {"type_for_const"}
// Dependencies: {}
# [doc = " Build the declared type of a const."] fn type_for_const < 'db > (db : & 'db dyn HirDatabase , def : ConstId) -> EarlyBinder < 'db , Ty < 'db > > { let resolver = def . resolver (db) ; let data = db . const_signature (def) ; let parent = def . loc (db) . container ; let mut ctx = TyLoweringContext :: new (db , & resolver , & data . store , def . into () , LifetimeElisionKind :: AnonymousReportError ,) ; ctx . set_lifetime_elision (LifetimeElisionKind :: for_const (ctx . interner , parent)) ; EarlyBinder :: bind (ctx . lower_ty (data . type_ref)) }
};
}
