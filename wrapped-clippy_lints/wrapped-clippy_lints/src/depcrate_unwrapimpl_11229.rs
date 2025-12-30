// Generated macro for impl_11229 (impl)
macro_rules! Depcrate_unwrapimpl_11229 {
() => {
// Module: crate::unwrap
// Provides: {"impl_11229"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Unwrap { fn check_fn (& mut self , cx : & LateContext < 'tcx > , kind : FnKind < 'tcx > , decl : & 'tcx FnDecl < '_ > , body : & 'tcx Body < '_ > , span : Span , fn_id : LocalDefId ,) { if span . from_expansion () { return ; } let mut v = UnwrappableVariablesVisitor { unwrappables : Vec :: new () , cx , msrv : self . msrv , } ; walk_fn (& mut v , kind , decl , body . id () , fn_id) ; } }
};
}
