// Generated macro for impl_2516 (impl)
macro_rules! Depcrate_functions_renamed_function_paramsimpl_2516 {
() => {
// Module: crate::functions::renamed_function_params
// Provides: {"impl_2516"}
// Dependencies: {}
impl RenamedFnArgs { # [doc = " Comparing between an iterator of default names and one with current names,"] # [doc = " then collect the ones that got renamed."] fn new < I1 , I2 > (default_idents : & mut I1 , current_idents : & mut I2) -> Self where I1 : Iterator < Item = Option < Ident > > , I2 : Iterator < Item = Option < Ident > > , { let mut renamed : Vec < (Span , String) > = vec ! [] ; debug_assert ! (default_idents . size_hint () == current_idents . size_hint ()) ; for (default_ident , current_ident) in iter :: zip (default_idents , current_idents) { let has_name_to_check = | ident : Option < Ident > | { ident . filter (| ident | ident . name != kw :: Underscore) . filter (| ident | ! ident . name . as_str () . starts_with ('_')) } ; if let Some (default_ident) = has_name_to_check (default_ident) && let Some (current_ident) = has_name_to_check (current_ident) && default_ident . name != current_ident . name { renamed . push ((current_ident . span , default_ident . to_string ())) ; } } Self (renamed) } fn multi_span (& self) -> MultiSpan { self . 0 . iter () . map (| (span , _) | span) . copied () . collect :: < Vec < Span > > () . into () } }
};
}
