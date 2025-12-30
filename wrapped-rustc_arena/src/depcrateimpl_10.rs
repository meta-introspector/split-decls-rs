// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
unsafe impl < # [may_dangle] T > Drop for ArenaChunk < T > { fn drop (& mut self) { unsafe { drop (Box :: from_raw (self . storage . as_mut ())) } } }
};
}
