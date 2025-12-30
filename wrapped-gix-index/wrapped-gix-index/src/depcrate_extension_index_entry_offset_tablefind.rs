// Generated macro for find (function)
macro_rules! Depcrate_extension_index_entry_offset_tablefind {
() => {
// Module: crate::extension::index_entry_offset_table
// Provides: {"find"}
// Dependencies: {}
pub fn find (extensions : & [u8] , object_hash : gix_hash :: Kind) -> Option < Vec < Offset > > { extension :: Iter :: new_without_checksum (extensions , object_hash) ? . find_map (| (sig , ext_data) | (sig == SIGNATURE) . then_some (ext_data)) . and_then (decode) }
};
}
