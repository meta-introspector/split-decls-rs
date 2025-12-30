// Generated macro for remove_lines (function)
macro_rules! Depcrate_query_engine_updateremove_lines {
() => {
// Module: crate::query::engine::update
// Provides: {"remove_lines"}
// Dependencies: {}
fn remove_lines (out : & mut Vec < FileChange > , path : & BStr , lines_counter : & AtomicUsize , id : gix :: Id < '_ >) { if let Ok (blob) = id . object () { let mut lines = LineStats :: default () ; let nl = blob . data . lines_with_terminator () . count () ; lines . removed += nl ; lines . before = nl ; lines_counter . fetch_add (nl , Ordering :: SeqCst) ; out . push (FileChange { relpath : path . to_owned () , mode : FileMode :: Removed , source_relpath : None , lines : Some (lines) , }) ; } }
};
}
