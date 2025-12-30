// Generated macro for parametrize (function)
macro_rules! Depcrate_renderparametrize {
() => {
// Module: crate::render
// Provides: {"parametrize"}
// Dependencies: {}
pub (crate) fn parametrize (mut test : ItemFn , info : RsTestInfo) -> TokenStream { let mut arguments_info = info . arguments . clone () ; test . apply_arguments (& mut arguments_info , & mut ()) ; let resolver_fixtures = resolver :: fixtures :: get (& info . arguments , info . data . fixtures ()) ; let rendered_cases = cases_data (& info , test . sig . ident . span ()) . map (| c | { CaseDataValues :: new (c . ident , c . attributes , Box :: new ((c . resolver , & resolver_fixtures)) , c . info ,) }) . map (| case | case . render (& test , & info)) . collect () ; test_group (test , rendered_cases) }
};
}
