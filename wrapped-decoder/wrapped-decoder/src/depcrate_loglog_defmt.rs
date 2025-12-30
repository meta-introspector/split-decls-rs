// Generated macro for log_defmt (function)
macro_rules! Depcrate_loglog_defmt {
() => {
// Module: crate::log
// Provides: {"log_defmt"}
// Dependencies: {}
# [doc = " Logs a defmt frame using the `log` facade."] pub fn log_defmt (frame : & Frame < '_ > , file : Option < & str > , line : Option < u32 > , module_path : Option < & str > ,) { let (timestamp , level) = timestamp_and_level_from_frame (frame) ; let target = format ! ("{}{}" , DEFMT_TARGET_MARKER , serde_json :: to_value (Payload { timestamp , level }) . unwrap ()) ; log :: logger () . log (& LogRecord :: builder () . args (format_args ! ("{}" , frame . display_message ())) . target (& target) . module_path (module_path) . file (file) . line (line) . build () ,) ; }
};
}
