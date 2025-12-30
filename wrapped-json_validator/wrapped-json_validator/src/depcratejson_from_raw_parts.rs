// Generated macro for json_from_raw_parts (function)
macro_rules! Depcratejson_from_raw_parts {
() => {
// Module: crate
// Provides: {"json_from_raw_parts"}
// Dependencies: {}
unsafe fn json_from_raw_parts (value : * const u8 , value_len : usize) -> Result < serde_json :: Value > { unsafe { if value . is_null () { return Err (E_POINTER . into ()) ; } let value = std :: slice :: from_raw_parts (value , value_len) ; let value = std :: str :: from_utf8 (value) . map_err (| _ | Error :: from (ERROR_NO_UNICODE_TRANSLATION)) ? ; serde_json :: from_str (value) . map_err (| error | Error :: new (E_INVALIDARG , format ! ("{error}"))) } }
};
}
