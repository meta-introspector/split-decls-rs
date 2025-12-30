// Generated macro for tuple_field_name (function)
macro_rules! Depcrate_debuginfo_metadatatuple_field_name {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"tuple_field_name"}
// Dependencies: {}
fn tuple_field_name (field_index : usize) -> Cow < 'static , str > { const TUPLE_FIELD_NAMES : [& 'static str ; 16] = ["__0" , "__1" , "__2" , "__3" , "__4" , "__5" , "__6" , "__7" , "__8" , "__9" , "__10" , "__11" , "__12" , "__13" , "__14" , "__15" ,] ; TUPLE_FIELD_NAMES . get (field_index) . map (| s | Cow :: from (* s)) . unwrap_or_else (| | Cow :: from (format ! ("__{field_index}"))) }
};
}
