// Generated macro for type_contains_some_param (function)
macro_rules! Depcrate_internals_genericstype_contains_some_param {
() => {
// Module: crate::internals::generics
// Provides: {"type_contains_some_param"}
// Dependencies: {}
# [cfg (feature = "schema")] pub fn type_contains_some_param (type_ : & Type , params : & HashSet < Ident >) -> bool { let mut find : FindTyParams = FindTyParams :: from_params (params . iter ()) ; find . visit_type_top_level (type_) ; find . at_least_one_hit () }
};
}
