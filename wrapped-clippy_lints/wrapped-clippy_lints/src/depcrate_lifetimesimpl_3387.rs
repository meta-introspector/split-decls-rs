// Generated macro for impl_3387 (impl)
macro_rules! Depcrate_lifetimesimpl_3387 {
() => {
// Module: crate::lifetimes
// Provides: {"impl_3387"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for BodyLifetimeChecker < 'tcx > { type Result = ControlFlow < () > ; type NestedFilter = middle_nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_lifetime (& mut self , lifetime : & 'tcx Lifetime) -> ControlFlow < () > { if ! lifetime . is_anonymous () && lifetime . ident . name != kw :: StaticLifetime { return ControlFlow :: Break (()) ; } ControlFlow :: Continue (()) } }
};
}
