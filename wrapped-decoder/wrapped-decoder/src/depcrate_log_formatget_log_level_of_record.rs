// Generated macro for get_log_level_of_record (function)
macro_rules! Depcrate_log_formatget_log_level_of_record {
() => {
// Module: crate::log::format
// Provides: {"get_log_level_of_record"}
// Dependencies: {}
fn get_log_level_of_record (record : & Record) -> Option < Level > { match record { Record :: Defmt (record) => record . level () , Record :: Host (record) => Some (record . level ()) , } }
};
}
