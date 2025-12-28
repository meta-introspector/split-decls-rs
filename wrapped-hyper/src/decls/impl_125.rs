macro_rules! deps {
    () => {
        ReasonPhrase!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl ReasonPhrase { # [doc = " Gets the reason phrase as bytes."] pub fn as_bytes (& self) -> & [u8] { & self . 0 } # [doc = " Converts a static byte slice to a reason phrase."] pub const fn from_static (reason : & 'static [u8]) -> Self { if find_invalid_byte (reason) . is_some () { panic ! ("invalid byte in static reason phrase") ; } Self (Bytes :: from_static (reason)) } # [doc = " Converts a `Bytes` directly into a `ReasonPhrase` without validating."] # [doc = ""] # [doc = " Use with care; invalid bytes in a reason phrase can cause serious security problems if"] # [doc = " emitted in a response."] # [cfg (feature = "client")] pub (crate) fn from_bytes_unchecked (reason : Bytes) -> Self { Self (reason) } }
    };
}

impl_125!()