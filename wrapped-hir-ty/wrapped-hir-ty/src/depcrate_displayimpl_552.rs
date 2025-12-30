// Generated macro for impl_552 (impl)
macro_rules! Depcrate_displayimpl_552 {
() => {
// Module: crate::display
// Provides: {"impl_552"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for Region < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self . kind () { RegionKind :: ReEarlyParam (param) => { let generics = generics (f . db , param . id . parent) ; let param_data = & generics [param . id . local_id] ; write ! (f , "{}" , param_data . name . display (f . db , f . edition ())) ? ; Ok (()) } RegionKind :: ReBound (BoundVarIndexKind :: Bound (db) , idx) => { write ! (f , "?{}.{}" , db . as_u32 () , idx . var . as_u32 ()) } RegionKind :: ReBound (BoundVarIndexKind :: Canonical , idx) => { write ! (f , "?c.{}" , idx . var . as_u32 ()) } RegionKind :: ReVar (_) => write ! (f , "_") , RegionKind :: ReStatic => write ! (f , "'static") , RegionKind :: ReError (..) => { if cfg ! (test) { write ! (f , "'?") } else { write ! (f , "'_") } } RegionKind :: ReErased => write ! (f , "'<erased>") , RegionKind :: RePlaceholder (_) => write ! (f , "<placeholder>") , RegionKind :: ReLateParam (_) => write ! (f , "<late-param>") , } } }
};
}
