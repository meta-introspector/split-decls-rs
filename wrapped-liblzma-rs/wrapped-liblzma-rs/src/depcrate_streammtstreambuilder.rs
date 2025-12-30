// Generated macro for MtStreamBuilder (struct)
macro_rules! Depcrate_streamMtStreamBuilder {
() => {
// Module: crate::stream
// Provides: {"MtStreamBuilder"}
// Dependencies: {}
# [doc = " Builder to create a multithreaded stream encoder."] # [cfg (feature = "parallel")] pub struct MtStreamBuilder { raw : liblzma_sys :: lzma_mt , filters : Option < Filters > , }
};
}
