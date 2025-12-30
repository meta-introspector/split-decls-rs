// Generated macro for closure_body (function)
macro_rules! Depcrateclosure_body {
() => {
// Module: crate
// Provides: {"closure_body"}
// Dependencies: {}
fn closure_body (signature : & Signature) -> syn :: Result < proc_macro2 :: TokenStream > { let input_types = signature . inputs . iter () . enumerate () . map (| (index , typed) | { let FnArg :: Typed (PatType { ty , .. }) = typed else { return Err (syn :: Error :: new (typed . span () , "`self` receiver is not accepted as test argument" ,)) ; } ; Fixture :: new (index , ty . clone ()) }) . collect :: < syn :: Result < Vec < Fixture > > > () ? ; let mut block = { let parameters = input_types . iter () . map (| fixture | & fixture . identifier) ; let test_name = & signature . ident ; let mut invocation = quote ! { # test_name (# (# parameters ,) *) } ; if signature . asyncness . is_some () { invocation = quote ! { # invocation . await } ; } if let ReturnType :: Default = signature . output { invocation = quote ! { { let () = # invocation ; googletest :: Result :: Ok (()) } } ; } invocation } ; for fixture in input_types . iter () . rev () { block = fixture . wrap_call (block) ; } Ok (block) }
};
}
