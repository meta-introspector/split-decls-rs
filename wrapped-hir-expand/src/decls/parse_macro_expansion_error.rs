macro_rules! deps {
    () => {
        ExpandDatabase!();
        MacroCallId!();
        ExpandResult!();
    };
}

macro_rules! parse_macro_expansion_error {
    () => {
        deps!();
        fn parse_macro_expansion_error (db : & dyn ExpandDatabase , macro_call_id : MacroCallId ,) -> Option < Arc < ExpandResult < Arc < [SyntaxError] > > > > { let e : ExpandResult < Arc < [SyntaxError] > > = db . parse_macro_expansion (macro_call_id) . map (| it | Arc :: from (it . 0 . errors ())) ; if e . value . is_empty () && e . err . is_none () { None } else { Some (Arc :: new (e)) } }
    };
}

parse_macro_expansion_error!()