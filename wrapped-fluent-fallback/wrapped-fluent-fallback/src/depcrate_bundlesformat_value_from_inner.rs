// Generated macro for format_value_from_inner (macro)
macro_rules! Depcrate_bundlesformat_value_from_inner {
() => {
// Module: crate::bundles
// Provides: {"format_value_from_inner"}
// Dependencies: {}
macro_rules ! format_value_from_inner { ($ step : expr , $ id : expr , $ args : expr , $ errors : expr) => { let mut found_message = false ; while let Some (bundle) = $ step { let bundle = bundle . as_ref () . unwrap_or_else (| (bundle , err) | { $ errors . extend (err . iter () . cloned () . map (Into :: into)) ; bundle }) ; if let Some (msg) = bundle . get_message ($ id) { found_message = true ; if let Some (value) = msg . value () { let mut format_errors = vec ! [] ; let result = bundle . format_pattern (value , $ args , & mut format_errors) ; if ! format_errors . is_empty () { $ errors . push (LocalizationError :: Resolver { id : $ id . to_string () , locale : bundle . locales [0] . clone () , errors : format_errors , }) ; } return Some (result) ; } else { $ errors . push (LocalizationError :: MissingValue { id : $ id . to_string () , locale : Some (bundle . locales [0] . clone ()) , }) ; } } else { $ errors . push (LocalizationError :: MissingMessage { id : $ id . to_string () , locale : Some (bundle . locales [0] . clone ()) , }) ; } } if found_message { $ errors . push (LocalizationError :: MissingValue { id : $ id . to_string () , locale : None , }) ; } else { $ errors . push (LocalizationError :: MissingMessage { id : $ id . to_string () , locale : None , }) ; } return None ; } ; }
};
}
