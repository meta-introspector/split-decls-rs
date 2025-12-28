macro_rules! deps {
    () => {
        ExprToSpannedString!();
    };
}

macro_rules! get_single_str_spanned_from_tts {
    () => {
        deps!();
        pub (crate) fn get_single_str_spanned_from_tts (cx : & mut ExtCtxt < '_ > , span : Span , tts : TokenStream , name : & str ,) -> ExpandResult < Result < (Symbol , Span) , ErrorGuaranteed > , () > { let ExpandResult :: Ready (ret) = get_single_expr_from_tts (cx , span , tts , name) else { return ExpandResult :: Retry (()) ; } ; let ret = match ret { Ok (ret) => ret , Err (e) => return ExpandResult :: Ready (Err (e)) , } ; expr_to_spanned_string (cx , ret , "argument must be a string literal") . map (| res | { res . map_err (| err | match err { Ok ((err , _)) => err . emit () , Err (guar) => guar , }) . map (| ExprToSpannedString { symbol , span , .. } | (symbol , span)) }) }
    };
}

get_single_str_spanned_from_tts!();