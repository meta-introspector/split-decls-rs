// Generated macro for matrix (function)
macro_rules! Depcrate_rendermatrix {
() => {
// Module: crate::render
// Provides: {"matrix"}
// Dependencies: {}
pub (crate) fn matrix (mut test : ItemFn , mut info : RsTestInfo) -> TokenStream { test . apply_arguments (& mut info . arguments , & mut ()) ; let span = test . sig . ident . span () ; let cases = cases_data (& info , span) . collect :: < Vec < _ > > () ; let resolver = resolver :: fixtures :: get (& info . arguments , info . data . fixtures ()) ; let rendered_cases = if cases . is_empty () { let list_values = info . data . list_values () . collect :: < Vec < _ > > () ; _matrix_recursive (& test , & list_values , & resolver , & [] , & info , & None) } else { cases . into_iter () . map (| c | { let list_values = info . data . list_values () . collect :: < Vec < _ > > () ; _matrix_recursive (& test , & list_values , & (& c . resolver , & resolver) , c . attributes , & info , & c . info ,) . wrap_by_mod (& c . ident) }) . collect () } ; test_group (test , rendered_cases) }
};
}
