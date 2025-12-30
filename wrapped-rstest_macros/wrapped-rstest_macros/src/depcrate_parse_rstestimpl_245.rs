// Generated macro for impl_245 (impl)
macro_rules! Depcrate_parse_rstestimpl_245 {
() => {
// Module: crate::parse::rstest
// Provides: {"impl_245"}
// Dependencies: {}
impl ExtendWithFunctionAttrs for RsTestData { fn extend_with_function_attrs (& mut self , item_fn : & mut ItemFn) -> Result < () , ErrorsVec > { let composed_tuple ! (fixtures , case_args , cases , value_list , files) = merge_errors ! (extract_fixtures (item_fn) , extract_case_args (item_fn) , extract_cases (item_fn) , extract_value_list (item_fn) , extract_files (item_fn)) ? ; self . items . extend (fixtures . into_iter () . map (| f | f . into ())) ; self . items . extend (case_args . into_iter () . map (| f | f . into ())) ; self . items . extend (cases . into_iter () . map (| f | f . into ())) ; self . items . extend (value_list . into_iter () . map (| f | f . into ())) ; self . items . extend (ValueListFromFiles :: default () . to_value_list (files) ? . into_iter () . map (| f | f . into ()) ,) ; Ok (()) } }
};
}
