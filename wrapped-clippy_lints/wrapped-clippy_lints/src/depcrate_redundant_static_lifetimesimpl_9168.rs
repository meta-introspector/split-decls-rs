// Generated macro for impl_9168 (impl)
macro_rules! Depcrate_redundant_static_lifetimesimpl_9168 {
() => {
// Module: crate::redundant_static_lifetimes
// Provides: {"impl_9168"}
// Dependencies: {}
impl EarlyLintPass for RedundantStaticLifetimes { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { if ! self . msrv . meets (msrvs :: STATIC_IN_CONST) { return ; } if ! item . span . from_expansion () { if let ItemKind :: Const (box ConstItem { ty : ref var_type , .. }) = item . kind { Self :: visit_type (var_type , cx , "constants have by default a `'static` lifetime") ; } if let ItemKind :: Static (box StaticItem { ty : ref var_type , .. }) = item . kind { Self :: visit_type (var_type , cx , "statics have by default a `'static` lifetime") ; } } } extract_msrv_attr ! () ; }
};
}
