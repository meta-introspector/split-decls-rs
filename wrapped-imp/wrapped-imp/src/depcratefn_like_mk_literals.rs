// Generated macro for fn_like_mk_literals (function)
macro_rules! Depcratefn_like_mk_literals {
() => {
// Module: crate
// Provides: {"fn_like_mk_literals"}
// Dependencies: {}
# [proc_macro] pub fn fn_like_mk_literals (_args : TokenStream) -> TokenStream { let trees : Vec < TokenTree > = vec ! [TokenTree :: from (Literal :: byte_string (b"byte_string")) , TokenTree :: from (Literal :: character ('c')) , TokenTree :: from (Literal :: string ("string")) , TokenTree :: from (Literal :: string ("-string")) , TokenTree :: from (Literal :: c_string (c"cstring")) , TokenTree :: from (Literal :: f64_suffixed (3.14)) , TokenTree :: from (Literal :: f64_suffixed (- 3.14)) , TokenTree :: from (Literal :: f64_unsuffixed (3.14)) , TokenTree :: from (Literal :: f64_unsuffixed (- 3.14)) , TokenTree :: from (Literal :: i64_suffixed (123)) , TokenTree :: from (Literal :: i64_suffixed (- 123)) , TokenTree :: from (Literal :: i64_unsuffixed (123)) , TokenTree :: from (Literal :: i64_unsuffixed (- 123)) ,] ; TokenStream :: from_iter (trees) }
};
}
