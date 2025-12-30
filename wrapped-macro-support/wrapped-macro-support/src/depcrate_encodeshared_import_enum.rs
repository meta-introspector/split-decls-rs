// Generated macro for shared_import_enum (function)
macro_rules! Depcrate_encodeshared_import_enum {
() => {
// Module: crate::encode
// Provides: {"shared_import_enum"}
// Dependencies: {}
fn shared_import_enum < 'a > (i : & 'a ast :: StringEnum , _intern : & 'a Interner) -> StringEnum < 'a > { StringEnum { name : & i . js_name , generate_typescript : i . generate_typescript , variant_values : i . variant_values . iter () . map (| x | & * * x) . collect () , comments : i . comments . iter () . map (| s | & * * s) . collect () , js_namespace : i . js_namespace . clone () , } }
};
}
