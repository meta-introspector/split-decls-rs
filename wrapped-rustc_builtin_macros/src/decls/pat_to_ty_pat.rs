macro_rules! pat_to_ty_pat {
    () => {
        fn pat_to_ty_pat (cx : & mut ExtCtxt < '_ > , pat : ast :: Pat) -> Box < TyPat > { let kind = match pat . kind { ast :: PatKind :: Range (start , end , include_end) => TyPatKind :: Range (start . map (| value | Box :: new (AnonConst { id : DUMMY_NODE_ID , value })) , end . map (| value | Box :: new (AnonConst { id : DUMMY_NODE_ID , value })) , include_end ,) , ast :: PatKind :: Or (variants) => { TyPatKind :: Or (variants . into_iter () . map (| pat | pat_to_ty_pat (cx , * pat)) . collect ()) } ast :: PatKind :: Err (guar) => TyPatKind :: Err (guar) , _ => TyPatKind :: Err (cx . dcx () . span_err (pat . span , "pattern not supported in pattern types")) , } ; ty_pat (kind , pat . span) }
    };
}

pat_to_ty_pat!()