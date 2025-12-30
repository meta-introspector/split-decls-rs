// Generated macro for impl_424 (impl)
macro_rules! Depcrateimpl_424 {
() => {
// Module: crate
// Provides: {"impl_424"}
// Dependencies: {}
# [cfg (feature = "encoder")] impl < W > CountingWriter < W > { fn new (inner : W) -> Self { Self { inner , bytes_written : 0 , } } fn bytes_written (& self) -> u64 { self . bytes_written } fn into_inner (self) -> W { self . inner } fn inner (& self) -> & W { & self . inner } fn inner_mut (& mut self) -> & mut W { & mut self . inner } }
};
}
