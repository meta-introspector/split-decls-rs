// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl EventHeader { # [doc = " Populate the event details based on the provided netlog file constants."] pub fn populate_strings (& mut self , constants : & constants :: Constants) { self . ty_string = constants . log_event_types_id_keyed [& self . ty] . clone () ; self . phase_string = constants . log_event_phase_id_keyed [& self . phase] . clone () ; } }
};
}
