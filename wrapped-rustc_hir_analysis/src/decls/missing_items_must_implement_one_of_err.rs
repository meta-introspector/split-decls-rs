macro_rules! deps {
    () => {
        MissingOneOfTraitItem!();
    };
}

macro_rules! missing_items_must_implement_one_of_err {
    () => {
        deps!();
        fn missing_items_must_implement_one_of_err (tcx : TyCtxt < '_ > , impl_span : Span , missing_items : & [Ident] , annotation_span : Option < Span > ,) { let missing_items_msg = missing_items . iter () . map (Ident :: to_string) . collect :: < Vec < _ > > () . join ("`, `") ; tcx . dcx () . emit_err (errors :: MissingOneOfTraitItem { span : impl_span , note : annotation_span , missing_items_msg , }) ; }
    };
}

missing_items_must_implement_one_of_err!()