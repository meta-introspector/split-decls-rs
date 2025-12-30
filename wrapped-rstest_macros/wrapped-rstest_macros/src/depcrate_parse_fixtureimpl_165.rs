// Generated macro for impl_165 (impl)
macro_rules! Depcrate_parse_fixtureimpl_165 {
() => {
// Module: crate::parse::fixture
// Provides: {"impl_165"}
// Dependencies: {}
impl ExtendWithFunctionAttrs for FixtureInfo { fn extend_with_function_attrs (& mut self , item_fn : & mut ItemFn ,) -> std :: result :: Result < () , ErrorsVec > { let composed_tuple ! (fixtures , defaults , default_return_type , partials_return_type , once , futures , global_awt) = merge_errors ! (extract_fixtures (item_fn) , extract_defaults (item_fn) , extract_default_return_type (item_fn) , extract_partials_return_type (item_fn) , extract_once (item_fn) , extract_futures (item_fn) , extract_global_awt (item_fn)) ? ; self . data . items . extend (fixtures . into_iter () . map (| f | f . into ()) . chain (defaults . into_iter () . map (| d | d . into ())) ,) ; if let Some (return_type) = default_return_type { self . attributes . set_default_return_type (return_type) ; } for (id , return_type) in partials_return_type { self . attributes . set_partial_return_type (id , return_type) ; } self . arguments . set_once (once) ; self . arguments . set_global_await (global_awt) ; self . arguments . set_futures (futures . into_iter ()) ; self . arguments . register_inner_destructored_idents_names (item_fn) ; Ok (()) } }
};
}
