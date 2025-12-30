// Generated macro for impl_485 (impl)
macro_rules! Depcrateimpl_485 {
() => {
// Module: crate
// Provides: {"impl_485"}
// Dependencies: {}
impl From < CompactString > for Box < str > { fn from (value : CompactString) -> Self { if value . is_heap_allocated () { value . into_string () . into_boxed_str () } else { Box :: from (value . as_str ()) } } }
};
}
