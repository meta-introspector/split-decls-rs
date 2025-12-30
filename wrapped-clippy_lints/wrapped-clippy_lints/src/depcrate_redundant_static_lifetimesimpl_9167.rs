// Generated macro for impl_9167 (impl)
macro_rules! Depcrate_redundant_static_lifetimesimpl_9167 {
() => {
// Module: crate::redundant_static_lifetimes
// Provides: {"impl_9167"}
// Dependencies: {}
impl RedundantStaticLifetimes { fn visit_type (ty : & Ty , cx : & EarlyContext < '_ > , reason : & 'static str) { match ty . kind { TyKind :: Array (ref ty , _) | TyKind :: Slice (ref ty) => { Self :: visit_type (ty , cx , reason) ; } , TyKind :: Tup (ref tup) => { for tup_ty in tup { Self :: visit_type (tup_ty , cx , reason) ; } } , TyKind :: Ref (ref optional_lifetime , ref borrow_type) => { if let Some (lifetime) = * optional_lifetime { match borrow_type . ty . kind { TyKind :: Path (..) | TyKind :: Slice (..) | TyKind :: Array (..) | TyKind :: Tup (..) => { if lifetime . ident . name == kw :: StaticLifetime { let snip = snippet (cx , borrow_type . ty . span , "<type>") ; let sugg = format ! ("&{}{snip}" , borrow_type . mutbl . prefix_str ()) ; span_lint_and_then (cx , REDUNDANT_STATIC_LIFETIMES , lifetime . ident . span , reason , | diag | { diag . span_suggestion (ty . span , "consider removing `'static`" , sugg , Applicability :: MachineApplicable ,) ; } ,) ; } } , _ => { } , } } Self :: visit_type (& borrow_type . ty , cx , reason) ; } , _ => { } , } } }
};
}
