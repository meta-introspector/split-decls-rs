// Generated macro for sanity_check_sig (function)
macro_rules! Depcrate_mockable_structsanity_check_sig {
() => {
// Module: crate::mockable_struct
// Provides: {"sanity_check_sig"}
// Dependencies: {}
fn sanity_check_sig (sig : & Signature) { for arg in sig . inputs . iter () { if let FnArg :: Typed (pt) = arg { if let Type :: ImplTrait (it) = pt . ty . as_ref () { let bounds = & it . bounds ; let s = format ! ("Mockall does not support \"impl trait\" in argument position.  Use \"T: {}\" instead" , quote ! (# bounds)) ; compile_error (it . span () , & s) ; } } } }
};
}
