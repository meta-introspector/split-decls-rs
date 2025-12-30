// Generated macro for generate_linking_tests (macro)
macro_rules! Depcrate_ffigenerate_linking_tests {
() => {
// Module: crate::ffi
// Provides: {"generate_linking_tests"}
// Dependencies: {}
macro_rules ! generate_linking_tests { { extern $ abi : literal { $ ($ (# [$ m : meta]) * $ v : vis fn $ name : ident ($ ($ (# [$ a_m : meta]) * $ a : ident : $ t : ty) ,* $ (,) ?) $ (-> $ r : ty) ?;) + } mod $ test_name : ident ; } => { extern $ abi { $ ($ (# [$ m]) * $ v fn $ name ($ ($ (# [$ a_m]) * $ a : $ t) ,*) $ (-> $ r) ?;) + } # [allow (deprecated)] # [cfg (test)] mod $ test_name { # [allow (unused)] use super ::*; $ ($ (# [$ m]) * # [test] fn $ name () { let f : unsafe extern $ abi fn ($ ($ (# [$ a_m]) * $ t) ,*) $ (-> $ r) ? = crate :: ffi ::$ name ; std :: println ! ("{:p}" , f) ; }) + } } ; }
};
}
