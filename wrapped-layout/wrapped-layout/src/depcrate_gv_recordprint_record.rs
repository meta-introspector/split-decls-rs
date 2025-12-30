// Generated macro for print_record (function)
macro_rules! Depcrate_gv_recordprint_record {
() => {
// Module: crate::gv::record
// Provides: {"print_record"}
// Dependencies: {}
pub fn print_record (rec : & RecordDef , indent : usize) { match rec { RecordDef :: Text (label , port) => { println ! ("\"{}\"" , label) ; if let Option :: Some (port) = port { println ! ("\"{}\"" , port) ; } } RecordDef :: Array (arr) => { print ! ("{}" , " " . repeat (indent)) ; println ! ("[") ; for elem in arr { print_record (elem , indent + 1) ; } print ! ("{}" , " " . repeat (indent)) ; println ! ("]") ; } } }
};
}
