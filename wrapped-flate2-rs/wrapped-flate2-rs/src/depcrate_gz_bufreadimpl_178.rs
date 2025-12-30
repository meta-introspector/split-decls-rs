// Generated macro for impl_178 (impl)
macro_rules! Depcrate_gz_bufreadimpl_178 {
() => {
// Module: crate::gz::bufread
// Provides: {"impl_178"}
// Dependencies: {}
impl < R : BufRead > GzEncoder < R > { # [doc = " Creates a new encoder which will use the given compression level."] # [doc = ""] # [doc = " The encoder is not configured specially for the emitted header. For"] # [doc = " header configuration, see the `GzBuilder` type."] # [doc = ""] # [doc = " The data read from the stream `r` will be compressed and available"] # [doc = " through the returned reader."] pub fn new (r : R , level : Compression) -> GzEncoder < R > { GzBuilder :: new () . buf_read (r , level) } fn read_footer (& mut self , into : & mut [u8]) -> io :: Result < usize > { if self . pos == 8 { return Ok (0) ; } let crc = self . inner . get_ref () . crc () ; let calced_crc_bytes = crc . sum () . to_le_bytes () ; let arr = [calced_crc_bytes [0] , calced_crc_bytes [1] , calced_crc_bytes [2] , calced_crc_bytes [3] , crc . amount () as u8 , (crc . amount () >> 8) as u8 , (crc . amount () >> 16) as u8 , (crc . amount () >> 24) as u8 ,] ; Ok (copy (into , & arr , & mut self . pos)) } }
};
}
