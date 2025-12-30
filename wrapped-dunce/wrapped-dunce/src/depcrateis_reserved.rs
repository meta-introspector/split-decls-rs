// Generated macro for is_reserved (function)
macro_rules! Depcrateis_reserved {
() => {
// Module: crate
// Provides: {"is_reserved"}
// Dependencies: {}
# [cfg (any (windows , test))] fn is_reserved < P : AsRef < OsStr > > (file_name : P) -> bool { if let Some (name) = Path :: new (& file_name) . file_stem () . and_then (| s | s . to_str () ? . split ('.') . next ()) { let trimmed = name . trim_end_matches (' ') ; return trimmed . len () <= 4 && RESERVED_NAMES . into_iter () . any (| name | trimmed . eq_ignore_ascii_case (name)) ; } false }
};
}
