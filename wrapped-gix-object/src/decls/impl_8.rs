macro_rules! deps {
    () => {
        Trailers!();
        BodyRef!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a > BodyRef < 'a > { # [doc = " Parse `body` bytes into the trailer and the actual body."] pub fn from_bytes (body : & 'a [u8]) -> Self { body . rfind (b"\n\n") . map (| pos | (2 , pos)) . or_else (| | body . rfind (b"\r\n\r\n") . map (| pos | (4 , pos))) . and_then (| (sep_len , pos) | { let trailer = & body [pos + sep_len ..] ; let body = & body [.. pos] ; Trailers { cursor : trailer } . next () . map (| _ | BodyRef { body_without_trailer : body . as_bstr () , start_of_trailer : trailer , }) }) . unwrap_or_else (| | BodyRef { body_without_trailer : body . as_bstr () , start_of_trailer : & [] , }) } # [doc = " Returns the body with the trailers stripped."] # [doc = ""] # [doc = " You can iterate trailers with the [`trailers()`][BodyRef::trailers()] method."] pub fn without_trailer (& self) -> & 'a BStr { self . body_without_trailer } # [doc = " Return an iterator over the trailers parsed from the last paragraph of the body. Maybe empty."] pub fn trailers (& self) -> Trailers < 'a > { Trailers { cursor : self . start_of_trailer , } } }
    };
}

impl_8!();