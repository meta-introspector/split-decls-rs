// Generated macro for provide (function)
macro_rules! Depcrate_limitsprovide {
() => {
// Module: crate::limits
// Provides: {"provide"}
// Dependencies: {}
pub (crate) fn provide (providers : & mut Providers) { providers . limits = | tcx , () | { let attrs = tcx . hir_krate_attrs () ; Limits { recursion_limit : get_recursion_limit (tcx . hir_krate_attrs ()) , move_size_limit : find_attr ! (attrs , AttributeKind :: MoveSizeLimit { limit , .. } => * limit) . unwrap_or (Limit :: new (tcx . sess . opts . unstable_opts . move_size_limit . unwrap_or (0))) , type_length_limit : find_attr ! (attrs , AttributeKind :: TypeLengthLimit { limit , .. } => * limit) . unwrap_or (Limit :: new (2usize . pow (24))) , pattern_complexity_limit : find_attr ! (attrs , AttributeKind :: PatternComplexityLimit { limit , .. } => * limit) . unwrap_or (Limit :: unlimited ()) , } } }
};
}
