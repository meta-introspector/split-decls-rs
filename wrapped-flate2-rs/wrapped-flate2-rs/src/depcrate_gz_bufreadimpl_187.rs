// Generated macro for impl_187 (impl)
macro_rules! Depcrate_gz_bufreadimpl_187 {
() => {
// Module: crate::gz::bufread
// Provides: {"impl_187"}
// Dependencies: {}
impl < R : BufRead > Read for GzDecoder < R > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { loop { match & mut self . state { GzState :: Header (parser) => { parser . parse (self . reader . get_mut () . get_mut ()) ? ; self . state = GzState :: Body (GzHeader :: from (mem :: take (parser))) ; } GzState :: Body (header) => { if into . is_empty () { return Ok (0) ; } match self . reader . read (into) ? { 0 => { self . state = GzState :: Finished (mem :: take (header) , 0 , [0 ; 8]) ; } n => { return Ok (n) ; } } } GzState :: Finished (header , pos , buf) => { if * pos < buf . len () { * pos += read_into (self . reader . get_mut () . get_mut () , & mut buf [* pos ..]) ? ; } else { let (crc , amt) = finish (buf) ; if crc != self . reader . crc () . sum () || amt != self . reader . crc () . amount () { self . state = GzState :: End (Some (mem :: take (header))) ; return Err (corrupt ()) ; } else if self . multi { let is_eof = self . reader . get_mut () . get_mut () . fill_buf () . map (| buf | buf . is_empty ()) ? ; if is_eof { self . state = GzState :: End (Some (mem :: take (header))) ; } else { self . reader . reset () ; self . reader . get_mut () . reset_data () ; self . state = GzState :: Header (GzHeaderParser :: new ()) } } else { self . state = GzState :: End (Some (mem :: take (header))) ; } } } GzState :: Err (err) => { let result = Err (mem :: replace (err , io :: ErrorKind :: Other . into ())) ; self . state = GzState :: End (None) ; return result ; } GzState :: End (_) => return Ok (0) , } } } }
};
}
