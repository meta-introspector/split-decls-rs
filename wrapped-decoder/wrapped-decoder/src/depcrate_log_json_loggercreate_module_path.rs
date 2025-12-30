// Generated macro for create_module_path (function)
macro_rules! Depcrate_log_json_loggercreate_module_path {
() => {
// Module: crate::log::json_logger
// Provides: {"create_module_path"}
// Dependencies: {}
fn create_module_path (module_path : Option < & str >) -> Option < ModulePath > { let mut path = module_path ? . split ("::") . collect :: < Vec < _ > > () ; if path . len () < 2 { return None ; } ; let function = path . pop () ? . to_string () ; let crate_name = path . remove (0) . to_string () ; Some (ModulePath { crate_name , modules : path . into_iter () . map (| a | a . to_string ()) . collect () , function , }) }
};
}
