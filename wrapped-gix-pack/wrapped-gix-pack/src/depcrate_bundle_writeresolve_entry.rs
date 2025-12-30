// Generated macro for resolve_entry (function)
macro_rules! Depcrate_bundle_writeresolve_entry {
() => {
// Module: crate::bundle::write
// Provides: {"resolve_entry"}
// Dependencies: {}
fn resolve_entry (range : data :: EntryRange , mapped_file : & memmap2 :: Mmap) -> Option < & [u8] > { mapped_file . get (range . start as usize .. range . end as usize) }
};
}
