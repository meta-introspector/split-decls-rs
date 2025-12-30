// Generated macro for impl_489 (impl)
macro_rules! Depcrateimpl_489 {
() => {
// Module: crate
// Provides: {"impl_489"}
// Dependencies: {}
impl From < CompactString > for alloc :: vec :: Vec < u8 > { fn from (value : CompactString) -> Self { if value . is_heap_allocated () { value . into_string () . into_bytes () } else { value . as_bytes () . to_vec () } } }
};
}
