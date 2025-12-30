// Generated macro for Now (module)
macro_rules! Depcrate_TemporalNow {
() => {
// Module: crate::Temporal
// Provides: {"Now"}
// Dependencies: {}
pub mod Now { use super :: * ; # [wasm_bindgen] extern "C" { # [wasm_bindgen (js_namespace = ["Temporal" , "Now"])] pub fn instant () -> Instant ; # [wasm_bindgen (js_namespace = ["Temporal" , "Now"] , js_name = zonedDateTime)] pub fn zoned_date_time (calendar : & str) -> ZonedDateTime ; # [wasm_bindgen (js_namespace = ["Temporal" , "Now"] , js_name = zonedDateTimeISO)] pub fn zoned_date_time_iso () -> ZonedDateTime ; # [wasm_bindgen (js_namespace = ["Temporal" , "Now"] , js_name = plainDate)] pub fn plain_date (calendar : & str) -> PlainDate ; # [wasm_bindgen (js_namespace = ["Temporal" , "Now"] , js_name = plainDateISO)] pub fn plain_date_iso () -> PlainDate ; # [wasm_bindgen (js_namespace = ["Temporal" , "Now"] , js_name = plainTimeISO)] pub fn plain_time_iso () -> PlainTime ; } }
};
}
