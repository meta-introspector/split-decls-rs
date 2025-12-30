// Generated macro for json_from_hstring (function)
macro_rules! Depcratejson_from_hstring {
() => {
// Module: crate
// Provides: {"json_from_hstring"}
// Dependencies: {}
fn json_from_hstring (value : & HSTRING) -> Result < serde_json :: Value > { let value = String :: try_from (value) ? ; serde_json :: from_str (& value) . map_err (| error | Error :: new (E_INVALIDARG , format ! ("{error}"))) }
};
}
