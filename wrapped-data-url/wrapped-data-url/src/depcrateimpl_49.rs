// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a > DataUrl < 'a > { # [doc = " <https://fetch.spec.whatwg.org/#data-url-processor>"] # [doc = " but starting from a string rather than a parsed `Url`, to avoid extra string copies."] pub fn process (input : & 'a str) -> Result < Self , DataUrlError > { use crate :: DataUrlError :: * ; let after_colon = pretend_parse_data_url (input) . ok_or (NotADataUrl) ? ; let (from_colon_to_comma , encoded_body_plus_fragment) = find_comma_before_fragment (after_colon) . ok_or (NoComma) ? ; let (mime_type , base64) = parse_header (from_colon_to_comma) ; Ok (DataUrl { mime_type , base64 , encoded_body_plus_fragment , }) } pub fn mime_type (& self) -> & mime :: Mime { & self . mime_type } # [doc = " Streaming-decode the data URL’s body to `write_body_bytes`,"] # [doc = " and return the URL’s fragment identifier if it has one."] pub fn decode < F , E > (& self , write_body_bytes : F ,) -> Result < Option < FragmentIdentifier < 'a > > , forgiving_base64 :: DecodeError < E > > where F : FnMut (& [u8]) -> Result < () , E > , { if self . base64 { decode_with_base64 (self . encoded_body_plus_fragment , write_body_bytes) } else { decode_without_base64 (self . encoded_body_plus_fragment , write_body_bytes) . map_err (forgiving_base64 :: DecodeError :: WriteError) } } # [doc = " Return the decoded body, and the URL’s fragment identifier if it has one."] pub fn decode_to_vec (& self ,) -> Result < (Vec < u8 > , Option < FragmentIdentifier < 'a > >) , forgiving_base64 :: InvalidBase64 > { let mut body = Vec :: new () ; let fragment = self . decode (| bytes | { body . extend_from_slice (bytes) ; Ok (()) }) ? ; Ok ((body , fragment)) } }
};
}
