// Generated macro for hir_fmt_generic_arguments (function)
macro_rules! Depcrate_displayhir_fmt_generic_arguments {
() => {
// Module: crate::display
// Provides: {"hir_fmt_generic_arguments"}
// Dependencies: {}
fn hir_fmt_generic_arguments < 'db > (f : & mut HirFormatter < '_ , 'db > , parameters : & [GenericArg < 'db >] , self_ : Option < Ty < 'db > > ,) -> Result < () , HirDisplayError > { let mut first = true ; let lifetime_offset = parameters . iter () . position (| arg | arg . region () . is_some ()) ; let (ty_or_const , lifetimes) = match lifetime_offset { Some (offset) => parameters . split_at (offset) , None => (parameters , & [] [..]) , } ; for generic_arg in lifetimes . iter () . chain (ty_or_const) { if ! mem :: take (& mut first) { write ! (f , ", ") ? ; } match self_ { self_ @ Some (_) if generic_arg . ty () == self_ => write ! (f , "Self") ? , _ => generic_arg . hir_fmt (f) ? , } } Ok (()) }
};
}
