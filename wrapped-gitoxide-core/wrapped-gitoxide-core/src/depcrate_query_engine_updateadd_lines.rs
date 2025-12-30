// Generated macro for add_lines (function)
macro_rules! Depcrate_query_engine_updateadd_lines {
() => {
// Module: crate::query::engine::update
// Provides: {"add_lines"}
// Dependencies: {}
fn add_lines (out : & mut Vec < FileChange > , path : & BStr , lines_counter : & AtomicUsize , id : gix :: Id < '_ >) { if let Ok (blob) = id . object () { let nl = blob . data . lines_with_terminator () . count () ; let mut lines = LineStats :: default () ; lines . added += nl ; lines . after = nl ; lines_counter . fetch_add (nl , Ordering :: SeqCst) ; out . push (FileChange { relpath : path . to_owned () , mode : FileMode :: Added , source_relpath : None , lines : Some (lines) , }) ; } }
};
}
