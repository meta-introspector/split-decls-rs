macro_rules! deps {
    () => {
        ObjectId!();
        Error!();
    };
}

macro_rules! decode {
    () => {
        deps!();
        # [allow (missing_docs)] pub mod decode { use std :: str :: FromStr ; use crate :: object_id :: ObjectId ; # [doc = " An error returned by [`ObjectId::from_hex()`][crate::ObjectId::from_hex()]"] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("A hash sized {0} hexadecimal characters is invalid")] InvalidHexEncodingLength (usize) , # [error ("Invalid character encountered")] Invalid , } # [doc = " Hash decoding"] impl ObjectId { # [doc = " Create an instance from a `buffer` of 40 bytes encoded with hexadecimal notation."] # [doc = ""] # [doc = " Such a buffer can be obtained using [`oid::write_hex_to(buffer)`][super::oid::write_hex_to()]"] pub fn from_hex (buffer : & [u8]) -> Result < ObjectId , Error > { match buffer . len () { 40 => Ok ({ ObjectId :: Sha1 ({ let mut buf = [0 ; 20] ; faster_hex :: hex_decode (buffer , & mut buf) . map_err (| err | match err { faster_hex :: Error :: InvalidChar | faster_hex :: Error :: Overflow => Error :: Invalid , faster_hex :: Error :: InvalidLength (_) => { unreachable ! ("BUG: This is already checked") } }) ? ; buf }) }) , len => Err (Error :: InvalidHexEncodingLength (len)) , } } } impl FromStr for ObjectId { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: from_hex (s . as_bytes ()) } } }
    };
}

decode!();