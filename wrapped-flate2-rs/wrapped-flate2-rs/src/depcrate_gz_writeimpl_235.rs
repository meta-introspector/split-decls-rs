// Generated macro for impl_235 (impl)
macro_rules! Depcrate_gz_writeimpl_235 {
() => {
// Module: crate::gz::write
// Provides: {"impl_235"}
// Dependencies: {}
impl < W : Write > Write for GzDecoder < W > { fn write (& mut self , mut buf : & [u8]) -> io :: Result < usize > { let buflen = buf . len () ; if self . header () . is_none () { match self . header_parser . parse (& mut buf) { Err (err) => { if err . kind () == io :: ErrorKind :: UnexpectedEof { Ok (buflen) } else { Err (err) } } Ok (_) => { debug_assert ! (self . header () . is_some ()) ; let n = buflen - buf . len () ; Ok (n) } } } else { let (n , status) = self . inner . write_with_status (buf) ? ; if status == Status :: StreamEnd && n < buf . len () && self . crc_bytes . len () < 8 { let remaining = buf . len () - n ; let crc_bytes = cmp :: min (remaining , CRC_BYTES_LEN - self . crc_bytes . len ()) ; self . crc_bytes . extend (& buf [n .. n + crc_bytes]) ; return Ok (n + crc_bytes) ; } Ok (n) } } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } }
};
}
