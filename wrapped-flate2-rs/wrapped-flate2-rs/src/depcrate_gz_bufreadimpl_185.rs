// Generated macro for impl_185 (impl)
macro_rules! Depcrate_gz_bufreadimpl_185 {
() => {
// Module: crate::gz::bufread
// Provides: {"impl_185"}
// Dependencies: {}
impl < R : BufRead > GzDecoder < R > { # [doc = " Creates a new decoder from the given reader, immediately parsing the"] # [doc = " gzip header."] pub fn new (mut r : R) -> GzDecoder < R > { let mut header_parser = GzHeaderParser :: new () ; let state = match header_parser . parse (& mut r) { Ok (_) => GzState :: Body (GzHeader :: from (header_parser)) , Err (ref err) if io :: ErrorKind :: WouldBlock == err . kind () => { GzState :: Header (header_parser) } Err (err) => GzState :: Err (err) , } ; GzDecoder { state , reader : CrcReader :: new (deflate :: bufread :: DeflateDecoder :: new (r)) , multi : false , } } fn multi (mut self , flag : bool) -> GzDecoder < R > { self . multi = flag ; self } }
};
}
