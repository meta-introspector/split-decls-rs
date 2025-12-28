macro_rules! ObsoleteAuto {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_obsolete_auto)] # [help] pub (crate) struct ObsoleteAuto { # [primary_span] pub span : Span , }
    };
}

ObsoleteAuto!()