// Generated macro for impl_70 (impl)
macro_rules! Depcrate_deflate_coreimpl_70 {
() => {
// Module: crate::deflate::core
// Provides: {"impl_70"}
// Dependencies: {}
impl Default for CompressorOxide { # [doc = " Initialize the compressor with a level of 4, zlib wrapper and"] # [doc = " the default strategy."] fn default () -> Self { CompressorOxide { lz : LZOxide :: new () , params : ParamsOxide :: new (DEFAULT_FLAGS) , huff : Box :: default () , dict : DictOxide :: new (DEFAULT_FLAGS) , } } }
};
}
