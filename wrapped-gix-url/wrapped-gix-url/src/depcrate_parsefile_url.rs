// Generated macro for file_url (function)
macro_rules! Depcrate_parsefile_url {
() => {
// Module: crate::parse
// Provides: {"file_url"}
// Dependencies: {}
pub (crate) fn file_url (input : & BStr , protocol_colon : usize) -> Result < crate :: Url , Error > { let input = input_to_utf8 (input , UrlKind :: Url) ? ; let input_after_protocol = & input [protocol_colon + "://" . len () ..] ; let Some (first_slash) = input_after_protocol . find ('/') . or_else (| | cfg ! (windows) . then (| | input_after_protocol . find ('\\')) . flatten ()) else { return Err (Error :: MissingRepositoryPath { url : input . to_owned () . into () , kind : UrlKind :: Url , }) ; } ; let windows_special_path = if cfg ! (windows) { let input_after_protocol = if first_slash == 0 { & input_after_protocol [1 ..] } else { input_after_protocol } ; if input_after_protocol . chars () . nth (1) == Some (':') { Some (input_after_protocol) } else { None } } else { None } ; let host = if windows_special_path . is_some () || first_slash == 0 { None } else { Some (& input_after_protocol [.. first_slash]) } ; let path = windows_special_path . unwrap_or (& input_after_protocol [first_slash ..]) ; Ok (crate :: Url { serialize_alternative_form : false , host : host . map (Into :: into) , .. local (path . into ()) ? }) }
};
}
