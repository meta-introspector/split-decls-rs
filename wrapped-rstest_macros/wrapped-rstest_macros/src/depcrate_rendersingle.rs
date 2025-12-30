// Generated macro for single (function)
macro_rules! Depcrate_rendersingle {
() => {
// Module: crate::render
// Provides: {"single"}
// Dependencies: {}
pub (crate) fn single (mut test : ItemFn , mut info : RsTestInfo) -> TokenStream { test . apply_arguments (& mut info . arguments , & mut ()) ; let resolver = resolver :: fixtures :: get (& info . arguments , info . data . fixtures ()) ; let args = test . sig . inputs . iter () . cloned () . collect :: < Vec < _ > > () ; let attrs = std :: mem :: take (& mut test . attrs) ; let asyncness = test . sig . asyncness ; single_test_case (& test . sig . ident , & test . sig . ident , & args , & attrs , & test . sig . output , asyncness , Some (& test) , resolver , & info , & test . sig . generics , & None ,) }
};
}
