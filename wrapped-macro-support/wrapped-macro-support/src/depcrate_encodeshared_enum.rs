// Generated macro for shared_enum (function)
macro_rules! Depcrate_encodeshared_enum {
() => {
// Module: crate::encode
// Provides: {"shared_enum"}
// Dependencies: {}
fn shared_enum < 'a > (e : & 'a ast :: Enum , intern : & 'a Interner) -> Enum < 'a > { Enum { name : & e . js_name , signed : e . signed , variants : e . variants . iter () . map (| v | shared_variant (v , intern)) . collect () , comments : e . comments . iter () . map (| s | & * * s) . collect () , generate_typescript : e . generate_typescript , js_namespace : e . js_namespace . clone () , } }
};
}
