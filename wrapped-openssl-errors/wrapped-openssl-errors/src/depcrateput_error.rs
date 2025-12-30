// Generated macro for put_error (macro)
macro_rules! Depcrateput_error {
() => {
// Module: crate
// Provides: {"put_error"}
// Dependencies: {}
# [doc = " Pushes an error onto the OpenSSL error stack."] # [doc = ""] # [doc = " A function and reason are required, and must be associated with the same error library. An additional formatted"] # [doc = " message string can also optionally be provided."] # [macro_export] macro_rules ! put_error { ($ function : expr , $ reason : expr) => { unsafe { $ crate :: __put_error ($ function , $ reason , concat ! (file ! () , "\0") , line ! () , $ crate :: export :: Option :: None ,) ; } } ; ($ function : expr , $ reason : expr , $ message : expr) => { unsafe { $ crate :: __put_error ($ function , $ reason , concat ! (file ! () , "\0") , line ! () , $ crate :: export :: Option :: Some ($ crate :: export :: Cow :: Borrowed (format_args ! (concat ! ($ message , "\0")) . as_str () . unwrap () ,)) ,) ; } } ; ($ function : expr , $ reason : expr , $ message : expr , $ ($ args : tt) *) => { unsafe { $ crate :: __put_error ($ function , $ reason , concat ! (file ! () , "\0") , line ! () , $ crate :: export :: Option :: Some ($ crate :: export :: Cow :: Owned (format ! (concat ! ($ message , "\0") , $ ($ args) *)) ,) ,) ; } } ; }
};
}
