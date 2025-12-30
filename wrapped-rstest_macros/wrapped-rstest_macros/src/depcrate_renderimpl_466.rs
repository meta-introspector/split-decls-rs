// Generated macro for impl_466 (impl)
macro_rules! Depcrate_renderimpl_466 {
() => {
// Module: crate::render
// Provides: {"impl_466"}
// Dependencies: {}
impl CaseDataValues < '_ > { fn render (self , testfn : & ItemFn , info : & RsTestInfo) -> TokenStream { let args = testfn . sig . inputs . iter () . cloned () . collect :: < Vec < _ > > () ; let mut attrs = testfn . attrs . clone () ; attrs . extend (self . attributes . iter () . cloned ()) ; let asyncness = testfn . sig . asyncness ; single_test_case (& self . ident , & testfn . sig . ident , & args , & attrs , & testfn . sig . output , asyncness , None , self . resolver , info , & testfn . sig . generics , & self . info ,) } }
};
}
