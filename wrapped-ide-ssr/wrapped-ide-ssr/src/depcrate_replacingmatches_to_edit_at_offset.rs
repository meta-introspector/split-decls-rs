// Generated macro for matches_to_edit_at_offset (function)
macro_rules! Depcrate_replacingmatches_to_edit_at_offset {
() => {
// Module: crate::replacing
// Provides: {"matches_to_edit_at_offset"}
// Dependencies: {}
fn matches_to_edit_at_offset < 'db > (db : & 'db dyn hir :: db :: ExpandDatabase , matches : & SsrMatches , file_src : & str , relative_start : TextSize , rules : & [ResolvedRule < 'db >] ,) -> TextEdit { let mut edit_builder = TextEdit :: builder () ; for m in & matches . matches { edit_builder . replace (m . range . range . checked_sub (relative_start) . unwrap () , render_replace (db , m , file_src , rules , m . range . file_id . edition (db)) ,) ; } edit_builder . finish () }
};
}
