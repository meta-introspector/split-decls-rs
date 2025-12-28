macro_rules! deps {
    () => {
        EmojiIdentifier!();
        FerrisIdentifier!();
    };
}

macro_rules! early_lint_checks {
    () => {
        deps!();
        fn early_lint_checks (tcx : TyCtxt < '_ > , () : ()) { let sess = tcx . sess ; let (resolver , krate) = & * tcx . resolver_for_lowering () . borrow () ; let mut lint_buffer = resolver . lint_buffer . steal () ; if sess . opts . unstable_opts . input_stats { input_stats :: print_ast_stats (tcx , krate) ; } sess . time ("complete_gated_feature_checking" , | | { rustc_ast_passes :: feature_gate :: check_crate (krate , sess , tcx . features ()) ; }) ; sess . psess . buffered_lints . with_lock (| buffered_lints | { info ! ("{} parse sess buffered_lints" , buffered_lints . len ()) ; for early_lint in buffered_lints . drain (..) { lint_buffer . add_early_lint (early_lint) ; } }) ; sess . psess . bad_unicode_identifiers . with_lock (| identifiers | { for (ident , mut spans) in identifiers . drain (..) { spans . sort () ; if ident == sym :: ferris { enum FerrisFix { SnakeCase , ScreamingSnakeCase , PascalCase , } impl FerrisFix { const fn as_str (self) -> & 'static str { match self { FerrisFix :: SnakeCase => "ferris" , FerrisFix :: ScreamingSnakeCase => "FERRIS" , FerrisFix :: PascalCase => "Ferris" , } } } let first_span = spans [0] ; let prev_source = sess . psess . source_map () . span_to_prev_source (first_span) ; let ferris_fix = prev_source . map_or (FerrisFix :: SnakeCase , | source | { let mut source_before_ferris = source . trim_end () . split_whitespace () . rev () ; match source_before_ferris . next () { Some ("struct" | "trait" | "mod" | "union" | "type" | "enum") => { FerrisFix :: PascalCase } Some ("const" | "static") => FerrisFix :: ScreamingSnakeCase , Some ("mut") if source_before_ferris . next () == Some ("static") => { FerrisFix :: ScreamingSnakeCase } _ => FerrisFix :: SnakeCase , } }) . as_str () ; sess . dcx () . emit_err (errors :: FerrisIdentifier { spans , first_span , ferris_fix }) ; } else { sess . dcx () . emit_err (errors :: EmojiIdentifier { spans , ident }) ; } } }) ; let lint_store = unerased_lint_store (tcx . sess) ; rustc_lint :: check_ast_node (sess , Some (tcx) , tcx . features () , false , lint_store , tcx . registered_tools (()) , Some (lint_buffer) , rustc_lint :: BuiltinCombinedEarlyLintPass :: new () , (& * * krate , & * krate . attrs) ,) }
    };
}

early_lint_checks!();