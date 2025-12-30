// Generated macro for extern_c_unwind (macro)
macro_rules! Depcrate_ffiextern_c_unwind {
() => {
// Module: crate::ffi
// Provides: {"extern_c_unwind"}
// Dependencies: {}
macro_rules ! extern_c_unwind { { $ ($ (# [$ m : meta]) * $ v : vis fn $ name : ident ($ ($ (# [$ a_m : meta]) * $ a : ident : $ t : ty) ,* $ (,) ?) $ (-> $ r : ty) ?;) + } => { generate_linking_tests ! { extern "C-unwind" { $ ($ (# [$ m]) * $ v fn $ name ($ ($ (# [$ a_m]) * $ a : $ t) ,*) $ (-> $ r) ?;) + } mod test_linkable_unwind ; } } ; }
};
}
