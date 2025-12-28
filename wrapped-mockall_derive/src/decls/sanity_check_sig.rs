macro_rules! sanity_check_sig {
    () => {
        fn sanity_check_sig (sig : & Signature) { for arg in sig . inputs . iter () { if let FnArg :: Typed (pt) = arg { if let Type :: ImplTrait (it) = pt . ty . as_ref () { let bounds = & it . bounds ; let s = format ! ("Mockall does not support \"impl trait\" in argument position.  Use \"T: {}\" instead" , quote ! (# bounds)) ; compile_error (it . span () , & s) ; } } } }
    };
}

sanity_check_sig!();