// Generated macro for is_unsafe_from_proc_macro (function)
macro_rules! Depcrate_undocumented_unsafe_blocksis_unsafe_from_proc_macro {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"is_unsafe_from_proc_macro"}
// Dependencies: {}
fn is_unsafe_from_proc_macro (cx : & LateContext < '_ > , span : Span) -> bool { let source_map = cx . sess () . source_map () ; let file_pos = source_map . lookup_byte_offset (span . lo ()) ; file_pos . sf . src . as_deref () . and_then (| src | src . get (file_pos . pos . to_usize () ..)) . is_none_or (| src | ! src . starts_with ("unsafe")) }
};
}
