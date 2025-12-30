// Generated macro for impl_560 (impl)
macro_rules! Depcrate_displayimpl_560 {
() => {
// Module: crate::display
// Provides: {"impl_560"}
// Dependencies: {}
impl < 'db > HirDisplayWithExpressionStore < 'db > for LifetimeRefId { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db > , store : & ExpressionStore ,) -> Result < () , HirDisplayError > { match & store [* self] { LifetimeRef :: Named (name) => write ! (f , "{}" , name . display (f . db , f . edition ())) , LifetimeRef :: Static => write ! (f , "'static") , LifetimeRef :: Placeholder => write ! (f , "'_") , LifetimeRef :: Error => write ! (f , "'{{error}}") , & LifetimeRef :: Param (lifetime_param_id) => { let generic_params = f . db . generic_params (lifetime_param_id . parent) ; write ! (f , "{}" , generic_params [lifetime_param_id . local_id] . name . display (f . db , f . edition ())) } } } }
};
}
