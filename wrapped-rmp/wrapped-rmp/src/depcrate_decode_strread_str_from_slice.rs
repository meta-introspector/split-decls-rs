// Generated macro for read_str_from_slice (function)
macro_rules! Depcrate_decode_strread_str_from_slice {
() => {
// Module: crate::decode::str
// Provides: {"read_str_from_slice"}
// Dependencies: {}
# [doc = " Attempts to read and decode a string value from the reader, returning a borrowed slice from it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rmp::encode::write_str;"] # [doc = " use rmp::decode::read_str_from_slice;"] # [doc = ""] # [doc = " let mut buf = Vec::new();"] # [doc = " write_str(&mut buf, \"Unpacking\").unwrap();"] # [doc = " write_str(&mut buf, \"multiple\").unwrap();"] # [doc = " write_str(&mut buf, \"strings\").unwrap();"] # [doc = ""] # [doc = " let mut chunks = Vec::new();"] # [doc = " let mut unparsed = &buf[..];"] # [doc = " while let Ok((chunk, tail)) = read_str_from_slice(unparsed) {"] # [doc = "     chunks.push(chunk);"] # [doc = "     unparsed = tail;"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(vec![\"Unpacking\", \"multiple\", \"strings\"], chunks);"] # [doc = " ```"] pub fn read_str_from_slice < T : ? Sized + AsRef < [u8] > > (buf : & T ,) -> Result < (& str , & [u8]) , DecodeStringError < '_ , super :: bytes :: BytesReadError > > { let buf = buf . as_ref () ; let (len , nread) = read_str_len_with_nread (& mut super :: Bytes :: new (buf)) ? ; let ulen = len as usize ; if buf [nread ..] . len () >= ulen { let (head , tail) = buf . split_at (nread + ulen) ; match from_utf8 (& head [nread ..]) { Ok (val) => Ok ((val , tail)) , Err (err) => Err (DecodeStringError :: InvalidUtf8 (buf , err)) , } } else { Err (DecodeStringError :: BufferSizeTooSmall (len)) } }
};
}
