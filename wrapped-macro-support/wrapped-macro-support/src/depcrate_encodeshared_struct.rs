// Generated macro for shared_struct (function)
macro_rules! Depcrate_encodeshared_struct {
() => {
// Module: crate::encode
// Provides: {"shared_struct"}
// Dependencies: {}
fn shared_struct < 'a > (s : & 'a ast :: Struct , intern : & 'a Interner) -> Struct < 'a > { Struct { name : & s . js_name , fields : s . fields . iter () . map (| s | shared_struct_field (s , intern)) . collect () , comments : s . comments . iter () . map (| s | & * * s) . collect () , is_inspectable : s . is_inspectable , generate_typescript : s . generate_typescript , js_namespace : s . js_namespace . clone () , } }
};
}
