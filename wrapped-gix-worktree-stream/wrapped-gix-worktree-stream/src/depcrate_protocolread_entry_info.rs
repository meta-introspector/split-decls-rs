// Generated macro for read_entry_info (function)
macro_rules! Depcrate_protocolread_entry_info {
() => {
// Module: crate::protocol
// Provides: {"read_entry_info"}
// Dependencies: {}
pub (crate) fn read_entry_info (read : & mut utils :: Read , path_buf : & mut BString ,) -> std :: io :: Result < (Option < usize > , gix_object :: tree :: EntryMode , gix_hash :: ObjectId) > { let mut buf = [0 ; std :: mem :: size_of :: < usize > () * 2 + 2] ; read . read_exact (& mut buf) ? ; let (path_len , rest) = buf . split_at (std :: mem :: size_of :: < usize > ()) ; let (stream_len , bytes) = rest . split_at (std :: mem :: size_of :: < usize > ()) ; let path_len = usize :: from_le_bytes (path_len . try_into () . expect ("valid")) ; let stream_size = usize :: from_le_bytes (stream_len . try_into () . expect ("valid")) ; let mode = byte_to_mode (bytes [0]) ; let hash_kind = byte_to_hash (bytes [1]) ; let mut hash = hash_kind . null () ; read . read_exact (hash . as_mut_slice ()) ? ; clear_and_set_len (path_buf , path_len) ? ; read . read_exact (path_buf) ? ; Ok (((stream_size != usize :: MAX) . then_some (stream_size) , mode , hash)) }
};
}
