// Generated macro for pos_allows_hyphen (function)
macro_rules! Depcrate_engine_completepos_allows_hyphen {
() => {
// Module: crate::engine::complete
// Provides: {"pos_allows_hyphen"}
// Dependencies: {}
fn pos_allows_hyphen (cmd : & clap :: Command , pos_index : usize) -> bool { cmd . get_positionals () . find (| a | a . get_index () == Some (pos_index)) . map (| p | p . is_allow_hyphen_values_set ()) . unwrap_or (false) }
};
}
