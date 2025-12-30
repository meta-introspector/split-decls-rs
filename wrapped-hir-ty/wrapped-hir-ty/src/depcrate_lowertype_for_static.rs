// Generated macro for type_for_static (function)
macro_rules! Depcrate_lowertype_for_static {
() => {
// Module: crate::lower
// Provides: {"type_for_static"}
// Dependencies: {}
# [doc = " Build the declared type of a static."] fn type_for_static < 'db > (db : & 'db dyn HirDatabase , def : StaticId) -> EarlyBinder < 'db , Ty < 'db > > { let resolver = def . resolver (db) ; let data = db . static_signature (def) ; let mut ctx = TyLoweringContext :: new (db , & resolver , & data . store , def . into () , LifetimeElisionKind :: AnonymousReportError ,) ; ctx . set_lifetime_elision (LifetimeElisionKind :: Elided (Region :: new_static (ctx . interner))) ; EarlyBinder :: bind (ctx . lower_ty (data . type_ref)) }
};
}
