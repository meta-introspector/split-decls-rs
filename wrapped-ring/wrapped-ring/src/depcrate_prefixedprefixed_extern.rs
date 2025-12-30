// Generated macro for prefixed_extern (macro)
macro_rules! Depcrate_prefixedprefixed_extern {
() => {
// Module: crate::prefixed
// Provides: {"prefixed_extern"}
// Dependencies: {}
macro_rules ! prefixed_extern { { $ ($ (# [$ meta : meta]) * $ vis : vis fn $ name : ident ($ ($ arg_pat : ident : $ arg_ty : ty) ,* $ (,) ?) $ (-> $ ret_ty : ty) ?;) + } => { extern "C" { $ (prefixed_item ! { link_name $ name { $ (# [$ meta]) * $ vis fn $ name ($ ($ arg_pat : $ arg_ty) ,*) $ (-> $ ret_ty) ?; } }) + } } ; { $ (# [$ meta : meta]) * $ vis : vis static $ name : ident : $ typ : ty ; } => { extern "C" { prefixed_item ! { link_name $ name { $ (# [$ meta]) * $ vis static $ name : $ typ ; } } } } ; { $ (# [$ meta : meta]) * $ vis : vis static mut $ name : ident : $ typ : ty ; } => { extern "C" { prefixed_item ! { link_name $ name { $ (# [$ meta]) * $ vis static mut $ name : $ typ ; } } } } ; }
};
}
