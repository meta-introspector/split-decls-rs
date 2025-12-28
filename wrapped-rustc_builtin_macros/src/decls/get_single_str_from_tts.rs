macro_rules! get_single_str_from_tts {
    () => {
        # [doc = " Interpreting `tts` as a comma-separated sequence of expressions,"] # [doc = " expect exactly one string literal, or emit an error and return `Err`."] pub (crate) fn get_single_str_from_tts (cx : & mut ExtCtxt < '_ > , span : Span , tts : TokenStream , name : & str ,) -> ExpandResult < Result < Symbol , ErrorGuaranteed > , () > { get_single_str_spanned_from_tts (cx , span , tts , name) . map (| res | res . map (| (s , _) | s)) }
    };
}

get_single_str_from_tts!()