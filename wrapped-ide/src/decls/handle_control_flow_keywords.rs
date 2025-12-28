macro_rules! deps {
    () => {
        HighlightedRange!();
        ReferenceSearchResult!();
    };
}

macro_rules! handle_control_flow_keywords {
    () => {
        deps!();
        fn handle_control_flow_keywords (sema : & Semantics < '_ , RootDatabase > , FilePosition { file_id , offset } : FilePosition ,) -> Option < ReferenceSearchResult > { let file = sema . parse_guess_edition (file_id) ; let edition = sema . attach_first_edition (file_id) . map (| it | it . edition (sema . db)) . unwrap_or (Edition :: CURRENT) ; let token = pick_best_token (file . syntax () . token_at_offset (offset) , | kind | match kind { _ if kind . is_keyword (edition) => 4 , T ! [=>] => 3 , _ => 1 , }) ? ; let references = match token . kind () { T ! [fn] | T ! [return] | T ! [try] => highlight_related :: highlight_exit_points (sema , token) , T ! [async] => highlight_related :: highlight_yield_points (sema , token) , T ! [loop] | T ! [while] | T ! [break] | T ! [continue] => { highlight_related :: highlight_break_points (sema , token) } T ! [for] if token . parent () . and_then (ast :: ForExpr :: cast) . is_some () => { highlight_related :: highlight_break_points (sema , token) } T ! [if] | T ! [=>] | T ! [match] => highlight_related :: highlight_branch_exit_points (sema , token) , _ => return None , } . into_iter () . map (| (file_id , ranges) | { let ranges = ranges . into_iter () . map (| HighlightedRange { range , category } | (range , category)) . collect () ; (file_id . file_id (sema . db) , ranges) }) . collect () ; Some (ReferenceSearchResult { declaration : None , references }) }
    };
}

handle_control_flow_keywords!();