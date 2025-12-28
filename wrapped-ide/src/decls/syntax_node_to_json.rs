macro_rules! deps {
    () => {
        TextPosition!();
        SyntaxTreeCtx!();
    };
}

macro_rules! syntax_node_to_json {
    () => {
        deps!();
        fn syntax_node_to_json (node : & SyntaxNode , ctx : & SyntaxTreeCtx) -> String { let mut result = String :: new () ; for event in node . preorder_with_tokens () { match event { WalkEvent :: Enter (it) => { let kind = it . kind () ; let (text_range , inner_range_str) = match & ctx . in_string { Some (in_string) => { let start_pos = TextPosition :: new (& ctx . line_index , it . text_range () . start ()) ; let end_pos = TextPosition :: new (& ctx . line_index , it . text_range () . end ()) ; let inner_start : u32 = it . text_range () . start () . into () ; let inner_end : u32 = it . text_range () . start () . into () ; let mut true_start = inner_start + in_string . offset ; let mut true_end = inner_end + in_string . offset ; for pos in & in_string . marker_positions { if * pos >= inner_end { break ; } true_start += 2 * (* pos < inner_start) as u32 ; true_end += 2 ; } let true_range = TextRange :: new (true_start . into () , true_end . into ()) ; (true_range , format ! (r#","istart":{start_pos},"iend":{end_pos}"# ,)) } None => (it . text_range () , "" . to_owned ()) , } ; let start = TextPosition :: new (& ctx . line_index , text_range . start ()) ; let end = TextPosition :: new (& ctx . line_index , text_range . end ()) ; match it { NodeOrToken :: Node (_) => { format_to ! (result , r#"{{"type":"Node","kind":"{kind:?}","start":{start},"end":{end}{inner_range_str},"children":["#) ; } NodeOrToken :: Token (token) => { let comma = if token . next_sibling_or_token () . is_some () { "," } else { "" } ; match parse_rust_string (token , ctx) { Some (parsed) => { format_to ! (result , r#"{{"type":"Node","kind":"{kind:?}","start":{start},"end":{end}{inner_range_str},"children":[{parsed}]}}{comma}"#) ; } None => format_to ! (result , r#"{{"type":"Token","kind":"{kind:?}","start":{start},"end":{end}{inner_range_str}}}{comma}"#) , } } } } WalkEvent :: Leave (it) => match it { NodeOrToken :: Node (node) => { let comma = if node . next_sibling_or_token () . is_some () { "," } else { "" } ; format_to ! (result , "]}}{comma}") } NodeOrToken :: Token (_) => () , } , } } result }
    };
}

syntax_node_to_json!();