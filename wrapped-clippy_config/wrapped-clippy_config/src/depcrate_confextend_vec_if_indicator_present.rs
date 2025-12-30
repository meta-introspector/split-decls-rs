// Generated macro for extend_vec_if_indicator_present (function)
macro_rules! Depcrate_confextend_vec_if_indicator_present {
() => {
// Module: crate::conf
// Provides: {"extend_vec_if_indicator_present"}
// Dependencies: {}
fn extend_vec_if_indicator_present (vec : & mut Vec < String > , default : & [& str]) { if vec . contains (& ".." . to_string ()) { vec . extend (default . iter () . map (ToString :: to_string)) ; } }
};
}
