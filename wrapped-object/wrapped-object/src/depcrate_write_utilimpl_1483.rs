// Generated macro for impl_1483 (impl)
macro_rules! Depcrate_write_utilimpl_1483 {
() => {
// Module: crate::write::util
// Provides: {"impl_1483"}
// Dependencies: {}
# [cfg (feature = "std")] impl < W > StreamingBuffer < W > { # [doc = " Create a new `StreamingBuffer` backed by the given writer."] pub fn new (writer : W) -> Self { StreamingBuffer { writer , len : 0 , result : Ok (()) , } } # [doc = " Unwraps this [`StreamingBuffer`] giving back the original writer."] pub fn into_inner (self) -> W { self . writer } # [doc = " Returns any error that occurred during writing."] pub fn result (& mut self) -> Result < () , io :: Error > { mem :: replace (& mut self . result , Ok (())) } }
};
}
