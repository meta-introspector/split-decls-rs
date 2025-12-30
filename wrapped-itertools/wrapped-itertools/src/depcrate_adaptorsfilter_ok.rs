// Generated macro for filter_ok (function)
macro_rules! Depcrate_adaptorsfilter_ok {
() => {
// Module: crate::adaptors
// Provides: {"filter_ok"}
// Dependencies: {}
# [doc = " Create a new `FilterOk` iterator."] pub fn filter_ok < I , F , T , E > (iter : I , f : F) -> FilterOk < I , F > where I : Iterator < Item = Result < T , E > > , F : FnMut (& T) -> bool , { FilterOk { iter , f } }
};
}
