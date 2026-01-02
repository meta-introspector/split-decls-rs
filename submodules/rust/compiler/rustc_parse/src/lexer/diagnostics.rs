mkuse!{use rustc_ast :: token :: Delimiter ;}
mkuse!{use rustc_errors :: Diag ;}
mkuse!{use rustc_session :: parse :: ParseSess ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_span :: source_map :: SourceMap ;}
mkuse!{use super :: UnmatchedDelim ;}
mkuse!{use crate :: errors :: MismatchedClosingDelimiter ;}
mkuse!{use crate :: pprust ;}
mkitem!{mkstruct!{# [derive (Default)] pub (super) struct TokenTreeDiagInfo { # [doc = " Stack of open delimiters and their spans. Used for error message."] pub open_delimiters : Vec < (Delimiter , Span) > , pub unmatched_delims : Vec < UnmatchedDelim > , # [doc = " Used only for error recovery when arriving to EOF with mismatched braces."] pub last_unclosed_found_span : Option < Span > , # [doc = " Collect empty block spans that might have been auto-inserted by editors."] pub empty_block_spans : Vec < Span > , # [doc = " Collect the spans of braces (Open, Close). Used only"] # [doc = " for detecting if blocks are empty and only braces."] pub matching_block_spans : Vec < (Span , Span) > , }}}

macro_rules! same_indentation_level_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function same_indentation_level in module {}", module_path!());
    };
}

mkfn!{
    same_indentation_level_introspect!();
    pub (super) fn same_indentation_level (sm : & SourceMap , open_sp : Span , close_sp : Span) -> bool { match (sm . span_to_margin (open_sp) , sm . span_to_margin (close_sp)) { (Some (open_padding) , Some (close_padding)) => open_padding == close_padding , _ => false , } }
}

macro_rules! report_missing_open_delim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_missing_open_delim in module {}", module_path!());
    };
}

mkfn!{
    report_missing_open_delim_introspect!();
    pub (super) fn report_missing_open_delim (err : & mut Diag < '_ > , unmatched_delims : & mut Vec < UnmatchedDelim > ,) -> bool { let mut reported_missing_open = false ; unmatched_delims . retain (| unmatch_brace | { if let Some (delim) = unmatch_brace . found_delim && matches ! (delim , Delimiter :: Parenthesis | Delimiter :: Bracket) { let missed_open = match delim { Delimiter :: Parenthesis => "(" , Delimiter :: Bracket => "[" , _ => unreachable ! () , } ; if let Some (unclosed_span) = unmatch_brace . unclosed_span { err . span_label (unclosed_span , "the nearest open delimiter") ; } err . span_label (unmatch_brace . found_span . shrink_to_lo () , format ! ("missing open `{missed_open}` for this delimiter") ,) ; reported_missing_open = true ; false } else { true } }) ; reported_missing_open }
}

macro_rules! report_suspicious_mismatch_block_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_suspicious_mismatch_block in module {}", module_path!());
    };
}

mkfn!{
    report_suspicious_mismatch_block_introspect!();
    pub (super) fn report_suspicious_mismatch_block (err : & mut Diag < '_ > , diag_info : & TokenTreeDiagInfo , sm : & SourceMap , delim : Delimiter ,) { let mut matched_spans : Vec < (Span , bool) > = diag_info . matching_block_spans . iter () . map (| & (open , close) | (open . with_hi (close . lo ()) , same_indentation_level (sm , open , close))) . collect () ; matched_spans . sort_by_key (| (span , _) | span . lo ()) ; for i in 0 .. matched_spans . len () { let (block_span , same_ident) = matched_spans [i] ; if same_ident { for j in i + 1 .. matched_spans . len () { let (inner_block , inner_same_ident) = matched_spans [j] ; if block_span . contains (inner_block) && ! inner_same_ident { matched_spans [j] = (inner_block , true) ; } } } } let candidate_span = matched_spans . into_iter () . rev () . find (| & (_ , same_ident) | ! same_ident) . map (| (span , _) | span) ; if let Some (block_span) = candidate_span { err . span_label (block_span . shrink_to_lo () , "this delimiter might not be properly closed...") ; err . span_label (block_span . shrink_to_hi () , "...as it matches this but it has different indentation" ,) ; if delim == Delimiter :: Brace { for span in diag_info . empty_block_spans . iter () { if block_span . contains (* span) { err . span_label (* span , "block is empty, you might have not meant to close it") ; break ; } } } } else { if let Some (parent) = diag_info . matching_block_spans . last () && diag_info . open_delimiters . last () . is_none () && diag_info . empty_block_spans . iter () . all (| & sp | sp != parent . 0 . to (parent . 1)) { err . span_label (parent . 0 , "this opening brace...") ; err . span_label (parent . 1 , "...matches this closing brace") ; } } }
}

macro_rules! make_errors_for_mismatched_closing_delims_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_errors_for_mismatched_closing_delims in module {}", module_path!());
    };
}

mkfn!{
    make_errors_for_mismatched_closing_delims_introspect!();
    pub (crate) fn make_errors_for_mismatched_closing_delims < 'psess > (unmatcheds : & [UnmatchedDelim] , psess : & 'psess ParseSess ,) -> Vec < Diag < 'psess > > { unmatcheds . iter () . filter_map (| unmatched | { let found_delim = unmatched . found_delim ? ; let mut spans = vec ! [unmatched . found_span] ; if let Some (sp) = unmatched . unclosed_span { spans . push (sp) ; } ; let err = psess . dcx () . create_err (MismatchedClosingDelimiter { spans , delimiter : pprust :: token_kind_to_string (& found_delim . as_close_token_kind ()) . to_string () , unmatched : unmatched . found_span , opening_candidate : unmatched . candidate_span , unclosed : unmatched . unclosed_span , }) ; Some (err) }) . collect () }
}