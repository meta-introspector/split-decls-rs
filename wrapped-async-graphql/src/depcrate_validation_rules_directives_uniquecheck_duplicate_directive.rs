// Generated macro for check_duplicate_directive (function)
macro_rules! Depcrate_validation_rules_directives_uniquecheck_duplicate_directive {
() => {
// Module: crate::validation::rules::directives_unique
// Provides: {"check_duplicate_directive"}
// Dependencies: {}
fn check_duplicate_directive (ctx : & mut VisitorContext < '_ > , directives : & [Positioned < Directive >]) { let mut exists = HashSet :: new () ; for directive in directives { let name = & directive . node . name . node ; if let Some (meta_directive) = ctx . registry . directives . get (name . as_str ()) { if ! meta_directive . is_repeatable { if exists . contains (name) { ctx . report_error (vec ! [directive . pos] , format ! ("Duplicate directive \"{}\"" , name) ,) ; continue ; } exists . insert (name) ; } } } }
};
}
