// Generated macro for forward_to_logger (function)
macro_rules! Depcrateforward_to_logger {
() => {
// Module: crate
// Provides: {"forward_to_logger"}
// Dependencies: {}
fn forward_to_logger (frame : & Frame , location_info : LocationInfo) { let (file , line , mod_path) = location_info ; defmt_decoder :: log :: log_defmt (frame , file . as_deref () , line , mod_path . as_deref ()) ; }
};
}
