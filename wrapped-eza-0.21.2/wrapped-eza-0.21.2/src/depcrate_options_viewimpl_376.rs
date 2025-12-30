// Generated macro for impl_376 (impl)
macro_rules! Depcrate_options_viewimpl_376 {
() => {
// Module: crate::options::view
// Provides: {"impl_376"}
// Dependencies: {}
impl TimeFormat { # [doc = " Determine how time should be formatted in timestamp columns."] fn deduce < V : Vars > (matches : & MatchedFlags < '_ > , vars : & V) -> Result < Self , OptionsError > { let word = if let Some (w) = matches . get (& flags :: TIME_STYLE) ? { w . to_os_string () } else { match vars . get (vars :: TIME_STYLE) { Some (ref t) if ! t . is_empty () => t . clone () , _ => return Ok (Self :: DefaultFormat) , } } ; match word . to_string_lossy () . as_ref () { "default" => Ok (Self :: DefaultFormat) , "relative" => Ok (Self :: Relative) , "iso" => Ok (Self :: ISOFormat) , "long-iso" => Ok (Self :: LongISO) , "full-iso" => Ok (Self :: FullISO) , fmt if fmt . starts_with ('+') => { let mut lines = fmt [1 ..] . lines () ; let empty_non_recent_format_msg = "Custom timestamp format is empty, \
                    please supply a chrono format string after the plus sign." ; let non_recent = lines . next () . expect (empty_non_recent_format_msg) ; let non_recent = if non_recent . is_empty () { panic ! ("{}" , empty_non_recent_format_msg) } else { non_recent . to_owned () } ; let empty_recent_format_msg = "Custom timestamp format for recent files is empty, \
                    please supply a chrono format string at the second line." ; let recent = lines . next () . map (| rec | { if rec . is_empty () { panic ! ("{}" , empty_recent_format_msg) } else { rec . to_owned () } }) ; Ok (Self :: Custom { non_recent , recent }) } _ => Err (OptionsError :: BadArgument (& flags :: TIME_STYLE , word)) , } } }
};
}
