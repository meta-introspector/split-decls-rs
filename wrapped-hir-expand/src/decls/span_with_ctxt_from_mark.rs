macro_rules! deps {
    () => {
        ExpandDatabase!();
        MacroCallId!();
    };
}

macro_rules! span_with_ctxt_from_mark {
    () => {
        deps!();
        fn span_with_ctxt_from_mark (db : & dyn ExpandDatabase , span : Span , expn_id : MacroCallId , transparency : Transparency , edition : Edition ,) -> Span { Span { ctx : apply_mark (db , SyntaxContext :: root (edition) , expn_id , transparency , edition) , .. span } }
    };
}

span_with_ctxt_from_mark!();