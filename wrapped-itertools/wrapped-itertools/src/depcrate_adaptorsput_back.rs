// Generated macro for put_back (function)
macro_rules! Depcrate_adaptorsput_back {
() => {
// Module: crate::adaptors
// Provides: {"put_back"}
// Dependencies: {}
# [doc = " Create an iterator where you can put back a single item"] pub fn put_back < I > (iterable : I) -> PutBack < I :: IntoIter > where I : IntoIterator , { PutBack { top : None , iter : iterable . into_iter () , } }
};
}
