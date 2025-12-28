macro_rules! deps {
    () => {
        ResolvedRule!();
        SsrMatches!();
    };
}

macro_rules! matches_to_edit {
    () => {
        deps!();
        # [doc = " Returns a text edit that will replace each match in `matches` with its corresponding replacement"] # [doc = " template. Placeholders in the template will have been substituted with whatever they matched to"] # [doc = " in the original code."] pub (crate) fn matches_to_edit < 'db > (db : & 'db dyn hir :: db :: ExpandDatabase , matches : & SsrMatches , file_src : & str , rules : & [ResolvedRule < 'db >] ,) -> TextEdit { matches_to_edit_at_offset (db , matches , file_src , 0 . into () , rules) }
    };
}

matches_to_edit!();