// Generated macro for impl_18 (impl)
macro_rules! Depcrate_file_accessimpl_18 {
() => {
// Module: crate::file::access
// Provides: {"impl_18"}
// Dependencies: {}
impl File { # [doc = " Returns the byte slice for the given commit in this file's Commit Data (CDAT) chunk."] pub (crate) fn commit_data_bytes (& self , pos : file :: Position) -> & [u8] { assert ! (pos . 0 < self . num_commits () , "expected lexicographical position less than {}, got {}" , self . num_commits () , pos . 0) ; let pos : usize = pos . 0 . try_into () . expect ("an architecture able to hold 32 bits of integer") ; let entry_size = self . hash_len + COMMIT_DATA_ENTRY_SIZE_SANS_HASH ; let start = self . commit_data_offset + (pos * entry_size) ; & self . data [start ..] [.. entry_size] } # [doc = " Returns the byte slice for this file's entire Extra Edge List (EDGE) chunk."] pub (crate) fn extra_edges_data (& self) -> Option < & [u8] > { Some (& self . data [self . extra_edges_list_range . clone () ?]) } }
};
}
