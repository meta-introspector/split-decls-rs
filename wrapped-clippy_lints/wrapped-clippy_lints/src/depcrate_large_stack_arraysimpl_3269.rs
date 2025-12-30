// Generated macro for impl_3269 (impl)
macro_rules! Depcrate_large_stack_arraysimpl_3269 {
() => {
// Module: crate::large_stack_arrays
// Provides: {"impl_3269"}
// Dependencies: {}
impl LargeStackArrays { pub fn new (conf : & 'static Conf) -> Self { Self { maximum_allowed_size : conf . array_size_threshold , prev_vec_macro_callsite : None , const_item_counter : Saturating (0) , } } # [doc = " Check if the given span of an expr is already in a `vec!` call."] fn is_from_vec_macro (& mut self , cx : & LateContext < '_ > , span : Span) -> bool { self . prev_vec_macro_callsite . is_some_and (| vec_mac | vec_mac . contains (span)) || { let res = macro_backtrace (span) . any (| mac | cx . tcx . is_diagnostic_item (sym :: vec_macro , mac . def_id)) ; if res { self . prev_vec_macro_callsite = Some (span . source_callsite ()) ; } res } } }
};
}
