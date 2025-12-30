// Generated macro for hir_fmt_tys (function)
macro_rules! Depcrate_displayhir_fmt_tys {
() => {
// Module: crate::display
// Provides: {"hir_fmt_tys"}
// Dependencies: {}
fn hir_fmt_tys < 'db > (f : & mut HirFormatter < '_ , 'db > , tys : & [Ty < 'db >] , self_ : Option < Ty < 'db > > ,) -> Result < () , HirDisplayError > { let mut first = true ; for ty in tys { if ! mem :: take (& mut first) { write ! (f , ", ") ? ; } match self_ { Some (self_) if * ty == self_ => write ! (f , "Self") ? , _ => ty . hir_fmt (f) ? , } } Ok (()) }
};
}
