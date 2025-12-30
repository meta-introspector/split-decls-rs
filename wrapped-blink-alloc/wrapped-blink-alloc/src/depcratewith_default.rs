// Generated macro for with_default (macro)
macro_rules! Depcratewith_default {
() => {
// Module: crate
// Provides: {"with_default"}
// Dependencies: {}
# [allow (unused)] macro_rules ! with_default { ($ (# [$ meta : meta]) * $ v : vis struct $ name : ident <$ ($ lt : lifetime ,) * $ ($ generic : ident $ (: $ bound : path $ (: $ bounds : path) *) ? $ (= +$ default : ty) ? $ (= $ default_type : ty) ?) ,+> { $ ($ (# [$ fmeta : meta]) * $ fvis : vis $ fname : ident : $ ftype : ty) ,* $ (,) ? }) => { $ (# [$ meta]) * $ v struct $ name <$ ($ lt ,) * $ ($ generic $ (: $ bound $ (+ $ bounds) *) ? $ (= $ default) ? $ (= $ default_type) ?) +> { $ ($ (# [$ fmeta]) * $ fvis $ fname : $ ftype ,) * } } ; }
};
}
