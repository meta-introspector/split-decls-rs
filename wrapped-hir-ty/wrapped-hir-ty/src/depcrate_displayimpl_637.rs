// Generated macro for impl_637 (impl)
macro_rules! Depcrate_displayimpl_637 {
() => {
// Module: crate::display
// Provides: {"impl_637"}
// Dependencies: {}
impl HirDisplayWithExpressionStore for LifetimeRefId { fn hir_fmt (& self , f : & mut HirFormatter < '_ > , store : & ExpressionStore ,) -> Result < () , HirDisplayError > { match & store [* self] { LifetimeRef :: Named (name) => write ! (f , "{}" , name . display (f . db , f . edition ())) , LifetimeRef :: Static => write ! (f , "'static") , LifetimeRef :: Placeholder => write ! (f , "'_") , LifetimeRef :: Error => write ! (f , "'{{error}}") , & LifetimeRef :: Param (lifetime_param_id) => { let generic_params = f . db . generic_params (lifetime_param_id . parent) ; write ! (f , "{}" , generic_params [lifetime_param_id . local_id] . name . display (f . db , f . edition ())) } } } }
};
}
