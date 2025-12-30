// Generated macro for estimate_path_storage_requirements_in_bytes (function)
macro_rules! Depcrate_decode_entriesestimate_path_storage_requirements_in_bytes {
() => {
// Module: crate::decode::entries
// Provides: {"estimate_path_storage_requirements_in_bytes"}
// Dependencies: {}
pub fn estimate_path_storage_requirements_in_bytes (num_entries : u32 , on_disk_size : usize , offset_to_extensions : Option < usize > , object_hash : gix_hash :: Kind , version : Version ,) -> usize { const fn on_disk_entry_sans_path (object_hash : gix_hash :: Kind) -> usize { 8 + 8 + (4 * 6) + 2 + object_hash . len_in_bytes () } match version { Version :: V3 | Version :: V2 => { let size_of_entries_block = offset_to_extensions . unwrap_or (on_disk_size) ; size_of_entries_block . saturating_sub (num_entries as usize * on_disk_entry_sans_path (object_hash)) . saturating_sub (header :: SIZE) } Version :: V4 => num_entries as usize * AVERAGE_V4_DELTA_PATH_LEN_IN_BYTES , } }
};
}
