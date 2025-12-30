// Generated macro for url (function)
macro_rules! Depcrate_parseurl {
() => {
// Module: crate::parse
// Provides: {"url"}
// Dependencies: {}
pub (crate) fn url (input : & BStr , protocol_end : usize) -> Result < crate :: Url , Error > { const MAX_LEN : usize = 1024 ; let bytes_to_path = input [protocol_end + "://" . len () ..] . iter () . filter (| b | ! b . is_ascii_whitespace ()) . skip_while (| b | * * b == b'/' || * * b == b'\\') . position (| b | * b == b'/') . unwrap_or (input . len () - protocol_end) ; if bytes_to_path > MAX_LEN || protocol_end > MAX_LEN { return Err (Error :: TooLong { truncated_url : input [.. (protocol_end + "://" . len () + MAX_LEN) . min (input . len ())] . into () , len : input . len () , }) ; } let (input , url) = input_to_utf8_and_url (input , UrlKind :: Url) ? ; let scheme = Scheme :: from (url . scheme . as_str ()) ; if matches ! (scheme , Scheme :: Git | Scheme :: Ssh) && url . path . is_empty () { return Err (Error :: MissingRepositoryPath { url : input . into () , kind : UrlKind :: Url , }) ; } let path = if url . path . is_empty () && matches ! (scheme , Scheme :: Http | Scheme :: Https) { "/" . into () } else { url . path . into () } ; Ok (crate :: Url { serialize_alternative_form : false , scheme , user : url_user (& url , UrlKind :: Url) ? , password : url . password . map (| s | percent_decoded_utf8 (s , UrlKind :: Url)) . transpose () ? , host : url . host , port : url . port , path , }) }
};
}
