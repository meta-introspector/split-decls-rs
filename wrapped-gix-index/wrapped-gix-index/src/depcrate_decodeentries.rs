// Generated macro for entries (function)
macro_rules! Depcrate_decodeentries {
() => {
// Module: crate::decode
// Provides: {"entries"}
// Dependencies: {}
fn entries (post_header_data : & [u8] , path_backing_buffer_size : usize , num_entries : u32 , object_hash : gix_hash :: Kind , version : Version ,) -> Result < (EntriesOutcome , & [u8]) , Error > { let mut entries = Vec :: with_capacity (num_entries as usize) ; let mut path_backing = Vec :: with_capacity (path_backing_buffer_size) ; entries :: chunk (post_header_data , & mut entries , & mut path_backing , num_entries , object_hash , version ,) . map (| (entries :: Outcome { is_sparse } , data) : (entries :: Outcome , & [u8]) | { (EntriesOutcome { entries , path_backing , is_sparse , } , data ,) }) }
};
}
