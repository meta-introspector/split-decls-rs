// Generated macro for impl_241 (impl)
macro_rules! Depcrate_parse_rstestimpl_241 {
() => {
// Module: crate::parse::rstest
// Provides: {"impl_241"}
// Dependencies: {}
impl ExtendWithFunctionAttrs for RsTestInfo { fn extend_with_function_attrs (& mut self , item_fn : & mut ItemFn) -> Result < () , ErrorsVec > { let (composed_tuple ! (excluded , _timeout , futures , global_awt , by_refs , ignores , contexts , test_attr) , _inner) = merge_errors ! (merge_errors ! (extract_excluded_trace (item_fn) , check_timeout_attrs (item_fn) , extract_futures (item_fn) , extract_global_awt (item_fn) , extract_by_ref (item_fn) , extract_ignores (item_fn) , extract_context (item_fn) , extract_test_attr (item_fn) ,) , self . data . extend_with_function_attrs (item_fn)) ? ; self . attributes . add_notraces (excluded) ; self . arguments . set_global_await (global_awt) ; self . arguments . set_futures (futures . into_iter ()) ; self . arguments . set_by_refs (by_refs . into_iter ()) ; self . arguments . set_ignores (ignores . into_iter ()) ; self . arguments . set_contexts (contexts . into_iter ()) ; self . arguments . set_test_attr (test_attr) ; self . arguments . register_inner_destructored_idents_names (item_fn) ; Ok (()) } }
};
}
