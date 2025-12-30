// Generated macro for without_default (macro)
macro_rules! Depcratewithout_default {
() => {
// Module: crate
// Provides: {"without_default"}
// Dependencies: {}
# [allow (unused)] macro_rules ! without_default { ($ (# [$ meta : meta]) * $ v : vis struct $ name : ident <$ ($ lt : lifetime ,) * $ ($ generic : ident $ (: $ bound : path $ (: $ bounds : path) *) ? $ (= +$ default : ty) ? $ (= $ default_type : ty) ?) ,+> { $ ($ (# [$ fmeta : meta]) * $ fvis : vis $ fname : ident : $ ftype : ty) ,* $ (,) ? }) => { $ (# [$ meta]) * $ v struct $ name <$ ($ lt ,) * $ ($ generic $ (: $ bound $ (+ $ bounds) *) ? $ (= $ default_type) ?) +> { $ ($ (# [$ fmeta]) * $ fvis $ fname : $ ftype ,) * } } ; }
};
}
