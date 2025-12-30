// Generated macro for impl_190 (impl)
macro_rules! Depcrate_data_entry_headerimpl_190 {
() => {
// Module: crate::data::entry::header
// Provides: {"impl_190"}
// Dependencies: {}
impl Header { # [doc = " Encode this header along the given `decompressed_size_in_bytes` into the `out` write stream for use within a data pack."] # [doc = ""] # [doc = " Returns the amount of bytes written to `out`."] # [doc = " `decompressed_size_in_bytes` is the full size in bytes of the object that this header represents"] pub fn write_to (& self , decompressed_size_in_bytes : u64 , out : & mut dyn io :: Write) -> io :: Result < usize > { let mut size = decompressed_size_in_bytes ; let mut written = 1 ; let mut c : u8 = (self . as_type_id () << 4) | (size as u8 & 0b0000_1111) ; size >>= 4 ; while size != 0 { out . write_all (& [c | 0b1000_0000]) ? ; written += 1 ; c = size as u8 & 0b0111_1111 ; size >>= 7 ; } out . write_all (& [c]) ? ; use Header :: * ; match self { RefDelta { base_id : oid } => { out . write_all (oid . as_slice ()) ? ; written += oid . as_slice () . len () ; } OfsDelta { base_distance } => { let mut buf = [0u8 ; 10] ; let buf = leb64_encode (* base_distance , & mut buf) ; out . write_all (buf) ? ; written += buf . len () ; } Blob | Tree | Commit | Tag => { } } Ok (written) } # [doc = " The size of the header in bytes when serialized"] pub fn size (& self , decompressed_size : u64) -> usize { self . write_to (decompressed_size , & mut io :: sink ()) . expect ("io::sink() to never fail") } }
};
}
