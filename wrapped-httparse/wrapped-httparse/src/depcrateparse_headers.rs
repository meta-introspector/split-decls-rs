// Generated macro for parse_headers (function)
macro_rules! Depcrateparse_headers {
() => {
// Module: crate
// Provides: {"parse_headers"}
// Dependencies: {}
# [doc = " Parse a buffer of bytes as headers."] # [doc = ""] # [doc = " The return value, if complete and successful, includes the index of the"] # [doc = " buffer that parsing stopped at, and a sliced reference to the parsed"] # [doc = " headers. The length of the slice will be equal to the number of properly"] # [doc = " parsed headers."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let buf = b\"Host: foo.bar\\nAccept: */*\\n\\nblah blah\";"] # [doc = " let mut headers = [httparse::EMPTY_HEADER; 4];"] # [doc = " assert_eq!(httparse::parse_headers(buf, &mut headers),"] # [doc = "            Ok(httparse::Status::Complete((27, &["] # [doc = "                httparse::Header { name: \"Host\", value: b\"foo.bar\" },"] # [doc = "                httparse::Header { name: \"Accept\", value: b\"*/*\" }"] # [doc = "            ][..]))));"] # [doc = " ```"] pub fn parse_headers < 'b : 'h , 'h > (src : & 'b [u8] , mut dst : & 'h mut [Header < 'b >] ,) -> Result < (usize , & 'h [Header < 'b >]) > { let mut iter = Bytes :: new (src) ; let pos = complete ! (parse_headers_iter (& mut dst , & mut iter , & HeaderParserConfig :: default ())) ; Ok (Status :: Complete ((pos , dst))) }
};
}
