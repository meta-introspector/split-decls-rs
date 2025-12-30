// Generated macro for get_rpaths_relative_to_output (function)
macro_rules! Depcrate_back_rpathget_rpaths_relative_to_output {
() => {
// Module: crate::back::rpath
// Provides: {"get_rpaths_relative_to_output"}
// Dependencies: {}
fn get_rpaths_relative_to_output (config : & RPathConfig < '_ >) -> Vec < OsString > { config . libs . iter () . map (| a | get_rpath_relative_to_output (config , a)) . collect () }
};
}
