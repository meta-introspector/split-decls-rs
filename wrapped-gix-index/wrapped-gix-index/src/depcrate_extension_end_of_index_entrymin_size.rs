// Generated macro for MIN_SIZE (const)
macro_rules! Depcrate_extension_end_of_index_entryMIN_SIZE {
() => {
// Module: crate::extension::end_of_index_entry
// Provides: {"MIN_SIZE"}
// Dependencies: {}
# [doc = " The minimal size of the extension, depending on the shortest hash."] pub const MIN_SIZE : usize = 4 + gix_hash :: Kind :: shortest () . len_in_bytes () ;
};
}
