// Generated macro for impl_590 (impl)
macro_rules! Depcrate_displayimpl_590 {
() => {
// Module: crate::display
// Provides: {"impl_590"}
// Dependencies: {}
impl HirFormatter < '_ > { fn start_location_link (& mut self , location : ModuleDefId) { self . fmt . start_location_link (location) ; } fn end_location_link (& mut self) { self . fmt . end_location_link () ; } fn format_bounds_with < T , F : FnOnce (& mut Self) -> T > (& mut self , target : ProjectionTy , format_bounds : F ,) -> T { match self . bounds_formatting_ctx { BoundsFormattingCtx :: Entered { ref mut projection_tys_met } => { projection_tys_met . insert (target) ; format_bounds (self) } BoundsFormattingCtx :: Exited => { let mut projection_tys_met = FxHashSet :: default () ; projection_tys_met . insert (target) ; self . bounds_formatting_ctx = BoundsFormattingCtx :: Entered { projection_tys_met } ; let res = format_bounds (self) ; self . bounds_formatting_ctx = BoundsFormattingCtx :: Exited ; res } } } fn render_lifetime (& self , lifetime : & Lifetime) -> bool { match self . display_lifetimes { DisplayLifetime :: Always => true , DisplayLifetime :: OnlyStatic => matches ! (*** lifetime . interned () , LifetimeData :: Static) , DisplayLifetime :: OnlyNamed => { matches ! (*** lifetime . interned () , LifetimeData :: Placeholder (_)) } DisplayLifetime :: OnlyNamedOrStatic => matches ! (*** lifetime . interned () , LifetimeData :: Static | LifetimeData :: Placeholder (_)) , DisplayLifetime :: Never => false , } } }
};
}
