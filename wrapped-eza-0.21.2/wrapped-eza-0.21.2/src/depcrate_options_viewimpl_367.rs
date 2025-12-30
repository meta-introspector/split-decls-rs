// Generated macro for impl_367 (impl)
macro_rules! Depcrate_options_viewimpl_367 {
() => {
// Module: crate::options::view
// Provides: {"impl_367"}
// Dependencies: {}
impl View { pub fn deduce < V : Vars > (matches : & MatchedFlags < '_ > , vars : & V) -> Result < Self , OptionsError > { let mode = Mode :: deduce (matches , vars) ? ; let deref_links = matches . has (& flags :: DEREF_LINKS) ? ; let follow_links = matches . has (& flags :: FOLLOW_LINKS) ? ; let total_size = matches . has (& flags :: TOTAL_SIZE) ? ; let width = TerminalWidth :: deduce (matches , vars) ? ; let file_style = FileStyle :: deduce (matches , vars , width . actual_terminal_width () . is_some ()) ? ; Ok (Self { mode , width , file_style , deref_links , follow_links , total_size , }) } }
};
}
