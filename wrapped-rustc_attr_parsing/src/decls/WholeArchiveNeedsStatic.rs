macro_rules! WholeArchiveNeedsStatic {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_whole_archive_needs_static)] pub (crate) struct WholeArchiveNeedsStatic { # [primary_span] pub span : Span , }
    };
}

WholeArchiveNeedsStatic!();