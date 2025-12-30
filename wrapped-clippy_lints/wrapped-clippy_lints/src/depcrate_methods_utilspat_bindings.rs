// Generated macro for pat_bindings (function)
macro_rules! Depcrate_methods_utilspat_bindings {
() => {
// Module: crate::methods::utils
// Provides: {"pat_bindings"}
// Dependencies: {}
# [doc = " Returns a vector of all `HirId`s bound by the pattern."] fn pat_bindings (pat : & Pat < '_ >) -> Vec < HirId > { let mut collector = usage :: ParamBindingIdCollector { binding_hir_ids : Vec :: new () , } ; collector . visit_pat (pat) ; collector . binding_hir_ids }
};
}
