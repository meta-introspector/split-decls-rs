// Generated macro for impl_628 (impl)
macro_rules! Depcrate_displayimpl_628 {
() => {
// Module: crate::display
// Provides: {"impl_628"}
// Dependencies: {}
impl HirDisplay for LifetimeData { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { match self { LifetimeData :: Placeholder (idx) => { let id = lt_from_placeholder_idx (f . db , * idx) ; let generics = generics (f . db , id . parent) ; let param_data = & generics [id . local_id] ; write ! (f , "{}" , param_data . name . display (f . db , f . edition ())) ? ; Ok (()) } LifetimeData :: BoundVar (idx) => idx . hir_fmt (f) , LifetimeData :: InferenceVar (_) => write ! (f , "_") , LifetimeData :: Static => write ! (f , "'static") , LifetimeData :: Error => { if cfg ! (test) { write ! (f , "'?") } else { write ! (f , "'_") } } LifetimeData :: Erased => write ! (f , "'<erased>") , LifetimeData :: Phantom (void , _) => match * void { } , } } }
};
}
