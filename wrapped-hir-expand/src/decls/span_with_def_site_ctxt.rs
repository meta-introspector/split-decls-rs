macro_rules! deps {
    () => {
        MacroCallId!();
        ExpandDatabase!();
    };
}

macro_rules! span_with_def_site_ctxt {
    () => {
        deps!();
        pub fn span_with_def_site_ctxt (db : & dyn ExpandDatabase , span : Span , expn_id : MacroCallId , edition : Edition ,) -> Span { span_with_ctxt_from_mark (db , span , expn_id , Transparency :: Opaque , edition) }
    };
}

span_with_def_site_ctxt!()