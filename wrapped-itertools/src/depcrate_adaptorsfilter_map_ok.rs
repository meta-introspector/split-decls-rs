// Generated macro for filter_map_ok (function)
macro_rules! Depcrate_adaptorsfilter_map_ok {
() => {
// Module: crate::adaptors
// Provides: {"filter_map_ok"}
// Dependencies: {}
# [doc = " Create a new `FilterMapOk` iterator."] pub fn filter_map_ok < I , F , T , U , E > (iter : I , f : F) -> FilterMapOk < I , F > where I : Iterator < Item = Result < T , E > > , F : FnMut (T) -> Option < U > , { FilterMapOk { iter , f } }
};
}
