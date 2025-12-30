// Generated macro for impl_563 (impl)
macro_rules! Depcrate_displayimpl_563 {
() => {
// Module: crate::display
// Provides: {"impl_563"}
// Dependencies: {}
impl < 'db > HirDisplayWithExpressionStore < 'db > for TypeBound { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db > , store : & ExpressionStore ,) -> Result < () , HirDisplayError > { match self { & TypeBound :: Path (path , modifier) => { match modifier { TraitBoundModifier :: None => () , TraitBoundModifier :: Maybe => write ! (f , "?") ? , } store [path] . hir_fmt (f , store) } TypeBound :: Lifetime (lifetime) => lifetime . hir_fmt (f , store) , TypeBound :: ForLifetime (lifetimes , path) => { let edition = f . edition () ; write ! (f , "for<{}> " , lifetimes . iter () . map (| it | it . display (f . db , edition)) . format (", ")) ? ; store [* path] . hir_fmt (f , store) } TypeBound :: Use (args) => { write ! (f , "use<") ? ; let edition = f . edition () ; let last = args . len () . saturating_sub (1) ; for (idx , arg) in args . iter () . enumerate () { match arg { UseArgRef :: Lifetime (lt) => lt . hir_fmt (f , store) ? , UseArgRef :: Name (n) => write ! (f , "{}" , n . display (f . db , edition)) ? , } if idx != last { write ! (f , ", ") ? ; } } write ! (f , "> ") } TypeBound :: Error => write ! (f , "{{error}}") , } } }
};
}
