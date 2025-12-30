// Generated macro for FxIndexMap (type)
macro_rules! DepcrateFxIndexMap {
() => {
// Module: crate
// Provides: {"FxIndexMap"}
// Dependencies: {}
pub type FxIndexMap < K , V > = indexmap :: IndexMap < K , V , std :: hash :: BuildHasherDefault < rustc_hash :: FxHasher > > ;
};
}
