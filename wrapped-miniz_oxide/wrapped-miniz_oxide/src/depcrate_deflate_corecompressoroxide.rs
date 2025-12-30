// Generated macro for CompressorOxide (struct)
macro_rules! Depcrate_deflate_coreCompressorOxide {
() => {
// Module: crate::deflate::core
// Provides: {"CompressorOxide"}
// Dependencies: {}
# [doc = " Main compression struct."] pub struct CompressorOxide { pub (crate) lz : LZOxide , pub (crate) params : ParamsOxide , # [doc = " Put HuffmanOxide on the heap with default trick to avoid"] # [doc = " excessive stack copies."] pub (crate) huff : Box < HuffmanOxide > , pub (crate) dict : DictOxide , }
};
}
