// Generated macro for impl_39 (impl)
macro_rules! Depcrate_property_test_optionsimpl_39 {
() => {
// Module: crate::property_test::options
// Provides: {"impl_39"}
// Dependencies: {}
impl Parse for Options { fn parse (input : syn :: parse :: ParseStream) -> syn :: Result < Self > { let pairs = Punctuated :: < MetaNameValue , Token ! [,] > :: parse_terminated (input) ? ; let mut errors = Vec :: new () ; let mut config = None ; for MetaNameValue { path , value , .. } in pairs { let path_string = path . get_ident () . map (Ident :: to_string) ; match path_string . as_deref () { None => errors . push (quote_spanned ! (path . span () => compile_error ! ("unknown argument"))) , Some ("config") => config = Some (value) , Some (other) => { let error_message = format ! ("unknown argument: {other}") ; let error_message = LitStr :: new (& error_message , other . span ()) ; let error = quote_spanned ! (other . span () => compile_error ! (# error_message)) ; errors . push (error) ; } } } Ok (Self { errors , config }) } }
};
}
