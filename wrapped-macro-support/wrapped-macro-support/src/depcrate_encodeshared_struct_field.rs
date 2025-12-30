// Generated macro for shared_struct_field (function)
macro_rules! Depcrate_encodeshared_struct_field {
() => {
// Module: crate::encode
// Provides: {"shared_struct_field"}
// Dependencies: {}
fn shared_struct_field < 'a > (s : & 'a ast :: StructField , _intern : & 'a Interner) -> StructField < 'a > { StructField { name : & s . js_name , readonly : s . readonly , comments : s . comments . iter () . map (| s | & * * s) . collect () , generate_typescript : s . generate_typescript , generate_jsdoc : s . generate_jsdoc , } }
};
}
