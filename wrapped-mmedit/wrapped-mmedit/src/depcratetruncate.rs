// Generated macro for truncate (function)
macro_rules! Depcratetruncate {
() => {
// Module: crate
// Provides: {"truncate"}
// Dependencies: {}
fn truncate (file_contents : & [u8]) -> Result < Vec < u8 > , Box < dyn Error + Send + Sync > > { let file_version = read_file_header (& file_contents , FILE_MAGIC_TOP_LEVEL , None , "top-level") ? ; if file_version < 7 || file_version > 8 { return Err (format ! ("File version {} is not support" , file_version) . into ()) ; } let paged_data = & file_contents [FILE_HEADER_SIZE ..] ; let mut truncated = file_contents [.. FILE_HEADER_SIZE] . to_vec () ; let mut event_page_emitted = false ; let mut pos = 0 ; while pos < paged_data . len () { let page_start = pos ; let tag = TryInto :: try_into (paged_data [pos]) . unwrap () ; let page_size = u32 :: from_le_bytes (paged_data [pos + 1 .. pos + 5] . try_into () . unwrap ()) as usize ; assert ! (page_size > 0) ; let page_end = page_start + 5 + page_size ; let page_bytes = & paged_data [page_start .. page_end] ; match tag { PageTag :: Events => { if ! event_page_emitted { truncated . extend_from_slice (page_bytes) ; event_page_emitted = true ; } } PageTag :: StringData | PageTag :: StringIndex => { truncated . extend_from_slice (page_bytes) ; } } pos = page_end ; } Ok (truncated) }
};
}
