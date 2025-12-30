// Generated macro for add_lines (function)
macro_rules! Depcrate_hours_utiladd_lines {
() => {
// Module: crate::hours::util
// Provides: {"add_lines"}
// Dependencies: {}
pub fn add_lines (line_stats : bool , lines_counter : & AtomicUsize , lines : & mut LineStats , id : gix :: Id < '_ >) { if let Some (Ok (blob)) = line_stats . then (| | id . object ()) { let nl = blob . data . lines_with_terminator () . count () ; lines . added += nl ; lines_counter . fetch_add (nl , Ordering :: Relaxed) ; } }
};
}
