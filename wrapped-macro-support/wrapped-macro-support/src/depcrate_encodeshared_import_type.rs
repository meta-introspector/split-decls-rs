// Generated macro for shared_import_type (function)
macro_rules! Depcrate_encodeshared_import_type {
() => {
// Module: crate::encode
// Provides: {"shared_import_type"}
// Dependencies: {}
fn shared_import_type < 'a > (i : & 'a ast :: ImportType , intern : & 'a Interner) -> ImportType < 'a > { ImportType { name : & i . js_name , instanceof_shim : & i . instanceof_shim , vendor_prefixes : i . vendor_prefixes . iter () . map (| x | intern . intern (x)) . collect () , } }
};
}
