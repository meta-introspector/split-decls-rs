// Generated macro for extern_c (macro)
macro_rules! Depcrate_ffiextern_c {
() => {
// Module: crate::ffi
// Provides: {"extern_c"}
// Dependencies: {}
macro_rules ! extern_c { { $ ($ (# [$ m : meta]) * $ v : vis fn $ name : ident ($ ($ (# [$ a_m : meta]) * $ a : ident : $ t : ty) ,* $ (,) ?) $ (-> $ r : ty) ?;) + } => { generate_linking_tests ! { extern "C" { $ ($ (# [$ m]) * $ v fn $ name ($ ($ (# [$ a_m]) * $ a : $ t) ,*) $ (-> $ r) ?;) + } mod test_linkable ; } } ; }
};
}
