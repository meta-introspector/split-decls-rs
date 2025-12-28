macro_rules! deps {
    () => {
        SsrMatches!();
        ResolvedRule!();
    };
}

macro_rules! matches_to_edit_at_offset {
    () => {
        deps!();
        fn matches_to_edit_at_offset < 'db > (db : & 'db dyn hir :: db :: ExpandDatabase , matches : & SsrMatches , file_src : & str , relative_start : TextSize , rules : & [ResolvedRule < 'db >] ,) -> TextEdit { let mut edit_builder = TextEdit :: builder () ; for m in & matches . matches { edit_builder . replace (m . range . range . checked_sub (relative_start) . unwrap () , render_replace (db , m , file_src , rules , m . range . file_id . edition (db)) ,) ; } edit_builder . finish () }
    };
}

matches_to_edit_at_offset!()