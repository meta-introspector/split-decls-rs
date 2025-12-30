// Generated macro for put_back_n (function)
macro_rules! Depcrate_put_back_n_implput_back_n {
() => {
// Module: crate::put_back_n_impl
// Provides: {"put_back_n"}
// Dependencies: {}
# [doc = " Create an iterator where you can put back multiple values to the front"] # [doc = " of the iteration."] # [doc = ""] # [doc = " Iterator element type is `I::Item`."] pub fn put_back_n < I > (iterable : I) -> PutBackN < I :: IntoIter > where I : IntoIterator , { PutBackN { top : Vec :: new () , iter : iterable . into_iter () , } }
};
}
