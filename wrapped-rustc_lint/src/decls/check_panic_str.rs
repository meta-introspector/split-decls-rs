macro_rules! deps {
    () => {
        NonFmtPanicUnused!();
        LateContext!();
        NonFmtPanicBraces!();
    };
}

macro_rules! check_panic_str {
    () => {
        deps!();
        fn check_panic_str < 'tcx > (cx : & LateContext < 'tcx > , f : & 'tcx hir :: Expr < 'tcx > , arg : & 'tcx hir :: Expr < 'tcx > , fmt : & str ,) { if ! fmt . contains (& ['{' , '}']) { return ; } let (span , _ , _) = panic_call (cx , f) ; let sm = cx . sess () . source_map () ; if span . in_external_macro (sm) && arg . span . in_external_macro (sm) { return ; } let fmt_span = arg . span . source_callsite () ; let (snippet , style) = match sm . span_to_snippet (fmt_span) { Ok (snippet) => { let style = snippet . strip_prefix ('r') . and_then (| s | s . find ('"')) ; (Some (snippet) , style) } Err (_) => (None , None) , } ; let mut fmt_parser = Parser :: new (fmt , style , snippet . clone () , false , ParseMode :: Format) ; let n_arguments = (& mut fmt_parser) . filter (| a | matches ! (a , Piece :: NextArgument (_))) . count () ; if n_arguments > 0 && fmt_parser . errors . is_empty () { let arg_spans : Vec < _ > = match & fmt_parser . arg_places [..] { [] => vec ! [fmt_span] , v => v . iter () . map (| span | fmt_span . from_inner (InnerSpan :: new (span . start , span . end))) . collect () , } ; cx . emit_span_lint (NON_FMT_PANICS , arg_spans , NonFmtPanicUnused { count : n_arguments , suggestion : is_arg_inside_call (arg . span , span) . then_some (arg . span) , } ,) ; } else { let brace_spans : Option < Vec < _ > > = snippet . filter (| s | s . starts_with ('"') || s . starts_with ("r#")) . map (| s | { s . char_indices () . filter (| & (_ , c) | c == '{' || c == '}') . map (| (i , _) | fmt_span . from_inner (InnerSpan { start : i , end : i + 1 })) . collect () }) ; let count = brace_spans . as_ref () . map (| v | v . len ()) . unwrap_or (2) ; cx . emit_span_lint (NON_FMT_PANICS , brace_spans . unwrap_or_else (| | vec ! [span]) , NonFmtPanicBraces { count , suggestion : is_arg_inside_call (arg . span , span) . then_some (arg . span . shrink_to_lo ()) , } ,) ; } }
    };
}

check_panic_str!()