// Generated macro for str_cmp (function)
macro_rules! Depcrate_stringstr_cmp {
() => {
// Module: crate::string
// Provides: {"str_cmp"}
// Dependencies: {}
# [test] fn str_cmp () { let cfstr = CFString :: new ("hello") ; assert_eq ! ("hello" , cfstr) ; assert_eq ! (cfstr , "hello") ; assert_ne ! (cfstr , "wrong") ; assert_ne ! ("wrong" , cfstr) ; let hello = String :: from ("hello") ; assert_eq ! (hello , cfstr) ; assert_eq ! (cfstr , hello) ; }
};
}
