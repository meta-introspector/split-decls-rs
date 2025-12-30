// Generated macro for hir_fmt_generic_args (function)
macro_rules! Depcrate_displayhir_fmt_generic_args {
() => {
// Module: crate::display
// Provides: {"hir_fmt_generic_args"}
// Dependencies: {}
fn hir_fmt_generic_args < 'db > (f : & mut HirFormatter < '_ , 'db > , parameters : & [GenericArg < 'db >] , generic_def : Option < hir_def :: GenericDefId > , self_ : Option < Ty < 'db > > ,) -> Result < () , HirDisplayError > { if parameters . is_empty () { return Ok (()) ; } let parameters_to_write = generic_args_sans_defaults (f , generic_def , parameters) ; if ! parameters_to_write . is_empty () { write ! (f , "<") ? ; hir_fmt_generic_arguments (f , parameters_to_write , self_) ? ; write ! (f , ">") ? ; } Ok (()) }
};
}
