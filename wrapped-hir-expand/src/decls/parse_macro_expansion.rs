macro_rules! deps {
    () => {
        ExpandResult!();
        ExpansionSpanMap!();
        MacroCallId!();
        ExpandDatabase!();
        CowArc!();
    };
}

macro_rules! parse_macro_expansion {
    () => {
        deps!();
        fn parse_macro_expansion (db : & dyn ExpandDatabase , macro_file : MacroCallId ,) -> ExpandResult < (Parse < SyntaxNode > , Arc < ExpansionSpanMap >) > { let _p = tracing :: info_span ! ("parse_macro_expansion") . entered () ; let loc = db . lookup_intern_macro_call (macro_file) ; let def_edition = loc . def . edition ; let expand_to = loc . expand_to () ; let mbe :: ValueResult { value : (tt , matched_arm) , err } = macro_expand (db , macro_file , loc) ; let (parse , mut rev_token_map) = token_tree_to_syntax_node (db , match & tt { CowArc :: Arc (it) => it , CowArc :: Owned (it) => it , } , expand_to , def_edition ,) ; rev_token_map . matched_arm = matched_arm ; ExpandResult { value : (parse , Arc :: new (rev_token_map)) , err } }
    };
}

parse_macro_expansion!()