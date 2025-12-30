// Generated macro for impl_941 (impl)
macro_rules! Depcrate_tz_concatenatedimpl_941 {
() => {
// Module: crate::tz::concatenated
// Provides: {"impl_941"}
// Dependencies: {}
impl Header { # [doc = " Reads the header from Android's concatenated TZif concatenated data"] # [doc = " file."] # [doc = ""] # [doc = " Basically, this gives us the version and some offsets for where to find"] # [doc = " data."] fn read < R : Read + ? Sized > (rdr : & R) -> Result < Header , Error > { let mut buf = [0 ; 12 + 3 * 4] ; rdr . read_exact_at (& mut buf , 0) . context ("failed to read concatenated TZif header") ? ; if & buf [.. 6] != b"tzdata" { return Err (err ! ("expected first 6 bytes of concatenated TZif header \
                 to be `tzdata`, but found `{found}`" , found = escape :: Bytes (& buf [.. 6]) ,)) ; } if buf [11] != 0 { return Err (err ! ("expected last byte of concatenated TZif header \
                 to be NUL, but found `{found}`" , found = escape :: Bytes (& buf [.. 12]) ,)) ; } let version = { let version = core :: str :: from_utf8 (& buf [6 .. 11]) . map_err (| _ | { err ! ("expected version in concatenated TZif header to \
                     be valid UTF-8, but found `{found}`" , found = escape :: Bytes (& buf [6 .. 11]) ,) }) ? ; ArrayStr :: new (version) . unwrap () } ; let index_offset = u64 :: from (read_be32 (& buf [12 .. 16])) ; let data_offset = u64 :: from (read_be32 (& buf [16 .. 20])) ; if index_offset > data_offset { return Err (err ! ("invalid index ({index_offset}) and data ({data_offset}) \
                 offsets, expected index offset to be less than or equal \
                 to data offset" ,)) ; } let header = Header { version , index_offset , data_offset } ; if header . index_len () % IndexEntry :: LEN != 0 { return Err (err ! ("length of index block is not a multiple {len}" , len = IndexEntry :: LEN ,)) ; } Ok (header) } # [doc = " Returns the length of the index section of the concatenated tzdb."] # [doc = ""] # [doc = " Beware of using this to create allocations. In theory, this should be"] # [doc = " trusted data, but the length can be any 32-bit integer. If it's used to"] # [doc = " create an allocation, it could potentially be up to 4GB."] fn index_len (& self) -> usize { let len = self . data_offset . checked_sub (self . index_offset) . unwrap () ; usize :: try_from (len) . unwrap_or (usize :: MAX) } }
};
}
