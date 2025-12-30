// Generated macro for impl_455 (impl)
macro_rules! Depcrate_renderimpl_455 {
() => {
// Module: crate::render
// Provides: {"impl_455"}
// Dependencies: {}
impl ValueList { fn render (& self , test : & ItemFn , resolver : & dyn Resolver , attrs : & [syn :: Attribute] , info : & RsTestInfo , case_info : & Option < CaseInfo > ,) -> TokenStream { let span = test . sig . ident . span () ; let test_cases = self . argument_data (resolver , info) . map (| (name , r) | { CaseDataValues :: new (Ident :: new (& name , span) , attrs , r , case_info . clone ()) }) . map (| test_case | test_case . render (test , info)) ; quote ! { # (# test_cases) * } } fn argument_data < 'a > (& 'a self , resolver : & 'a dyn Resolver , info : & 'a RsTestInfo ,) -> impl Iterator < Item = (String , ArgumentDataResolver < 'a >) > + 'a { let max_len = self . values . len () ; self . values . iter () . enumerate () . map (move | (index , value) | { let description = sanitize_ident (& value . description ()) ; let arg = info . arguments . inner_pat (& self . arg) ; let arg_name = arg . maybe_ident () . expect ("BUG: Here all arguments should be PatIdent types") . to_string () ; let name = format ! ("{}_{:0len$}_{description:.64}" , arg_name , index + 1 , len = max_len . display_len ()) ; let resolver_this = (arg . clone () , value . expr . clone ()) ; (name , Box :: new ((resolver , resolver_this))) }) } }
};
}
