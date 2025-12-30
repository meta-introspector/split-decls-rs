// Generated macro for get_instr_profile_output_path (function)
macro_rules! Depcrate_back_writeget_instr_profile_output_path {
() => {
// Module: crate::back::write
// Provides: {"get_instr_profile_output_path"}
// Dependencies: {}
fn get_instr_profile_output_path (config : & ModuleConfig) -> Option < CString > { config . instrument_coverage . then (| | c"default_%m_%p.profraw" . to_owned ()) }
};
}
