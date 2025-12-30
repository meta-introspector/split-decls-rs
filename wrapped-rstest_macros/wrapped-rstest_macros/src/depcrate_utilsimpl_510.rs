// Generated macro for impl_510 (impl)
macro_rules! Depcrate_utilsimpl_510 {
() => {
// Module: crate::utils
// Provides: {"impl_510"}
// Dependencies: {}
impl SearchSimpleTypeName { fn take (self) -> HashSet < Ident > { self . 0 } fn visit_inputs < 'a > (& mut self , inputs : impl Iterator < Item = & 'a FnArg >) { use syn :: visit :: Visit ; inputs . for_each (| fn_arg | self . visit_fn_arg (fn_arg)) ; } fn visit_output (& mut self , output : & ReturnType) { use syn :: visit :: Visit ; self . visit_return_type (output) ; } fn collect_from_type_param (tp : & syn :: TypeParam) -> Self { let mut s : Self = Default :: default () ; use syn :: visit :: Visit ; s . visit_type_param (tp) ; s } fn collect_from_where_predicate (wp : & syn :: WherePredicate) -> Self { let mut s : Self = Default :: default () ; use syn :: visit :: Visit ; s . visit_where_predicate (wp) ; s } }
};
}
