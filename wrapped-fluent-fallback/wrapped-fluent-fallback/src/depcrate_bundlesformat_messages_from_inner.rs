// Generated macro for format_messages_from_inner (macro)
macro_rules! Depcrate_bundlesformat_messages_from_inner {
() => {
// Module: crate::bundles
// Provides: {"format_messages_from_inner"}
// Dependencies: {}
macro_rules ! format_messages_from_inner { ($ step : expr , $ keys : expr , $ errors : expr) => { let mut result = vec ! [None ; $ keys . len ()] ; let mut is_complete = false ; while let Some (bundle) = $ step { let bundle = bundle . as_ref () . unwrap_or_else (| (bundle , err) | { $ errors . extend (err . iter () . cloned () . map (Into :: into)) ; bundle }) ; let mut has_missing = false ; for (key , cell) in $ keys . iter () . zip (& mut result) . filter (| (_ , cell) | cell . is_none ()) { let mut format_errors = vec ! [] ; let msg = Self :: format_message_from_bundle (bundle , key , & mut format_errors) ; if msg . is_none () { has_missing = true ; $ errors . push (LocalizationError :: MissingMessage { id : key . id . to_string () , locale : Some (bundle . locales [0] . clone ()) , }) ; } else if ! format_errors . is_empty () { $ errors . push (LocalizationError :: Resolver { id : key . id . to_string () , locale : bundle . locales . get (0) . cloned () . unwrap () , errors : format_errors , }) ; } * cell = msg ; } if ! has_missing { is_complete = true ; break ; } } if ! is_complete { for (key , _) in $ keys . iter () . zip (& mut result) . filter (| (_ , cell) | cell . is_none ()) { $ errors . push (LocalizationError :: MissingMessage { id : key . id . to_string () , locale : None , }) ; } } return result ; } ; }
};
}
