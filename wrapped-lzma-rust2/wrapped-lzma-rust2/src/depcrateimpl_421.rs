// Generated macro for impl_421 (impl)
macro_rules! Depcrateimpl_421 {
() => {
// Module: crate
// Provides: {"impl_421"}
// Dependencies: {}
impl < R > CountingReader < R > { fn new (inner : R) -> Self { Self { inner , bytes_read : 0 , } } fn with_count (inner : R , bytes_read : u64) -> Self { Self { inner , bytes_read } } fn bytes_read (& self) -> u64 { self . bytes_read } fn into_inner (self) -> R { self . inner } fn inner (& self) -> & R { & self . inner } fn inner_mut (& mut self) -> & mut R { & mut self . inner } }
};
}
