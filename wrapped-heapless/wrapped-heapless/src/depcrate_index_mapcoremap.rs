// Generated macro for CoreMap (struct)
macro_rules! Depcrate_index_mapCoreMap {
() => {
// Module: crate::index_map
// Provides: {"CoreMap"}
// Dependencies: {}
# [cfg_attr (feature = "zeroize" , derive (Zeroize) , zeroize (bound = "K: Zeroize, V: Zeroize"))] struct CoreMap < K , V , const N : usize > { entries : Vec < Bucket < K , V > , N , usize > , indices : [Option < Pos > ; N] , }
};
}
