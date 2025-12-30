// Generated macro for impl_10847 (impl)
macro_rules! Depcrate_unwrapimpl_10847 {
() => {
// Module: crate::unwrap
// Provides: {"impl_10847"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Unwrap { fn check_fn (& mut self , cx : & LateContext < 'tcx > , kind : FnKind < 'tcx > , decl : & 'tcx FnDecl < '_ > , body : & 'tcx Body < '_ > , span : Span , fn_id : LocalDefId ,) { if span . from_expansion () { return ; } let mut v = UnwrappableVariablesVisitor { unwrappables : Vec :: new () , cx , } ; walk_fn (& mut v , kind , decl , body . id () , fn_id) ; } }
};
}
