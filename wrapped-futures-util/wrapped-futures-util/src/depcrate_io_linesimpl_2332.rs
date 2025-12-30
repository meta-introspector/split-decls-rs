// Generated macro for impl_2332 (impl)
macro_rules! Depcrate_io_linesimpl_2332 {
() => {
// Module: crate::io::lines
// Provides: {"impl_2332"}
// Dependencies: {}
impl < R : AsyncBufRead > Lines < R > { pub (super) fn new (reader : R) -> Self { Self { reader , buf : String :: new () , bytes : Vec :: new () , read : 0 } } }
};
}
