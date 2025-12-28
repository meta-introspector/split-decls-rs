macro_rules! deps {
    () => {
        DefaultHasArg!();
        MultipleDefaultAttrs!();
        MultipleDefaultAttrsSugg!();
    };
}

macro_rules! validate_default_attribute {
    () => {
        deps!();
        fn validate_default_attribute (cx : & ExtCtxt < '_ > , default_variant : & rustc_ast :: Variant ,) -> Result < () , ErrorGuaranteed > { let attrs : SmallVec < [_ ; 1] > = attr :: filter_by_name (& default_variant . attrs , kw :: Default) . collect () ; let attr = match attrs . as_slice () { [attr] => attr , [] => cx . dcx () . bug ("this method must only be called with a variant that has a `#[default]` attribute" ,) , [first , rest @ ..] => { let sugg = errors :: MultipleDefaultAttrsSugg { spans : rest . iter () . map (| attr | attr . span) . collect () , } ; let guar = cx . dcx () . emit_err (errors :: MultipleDefaultAttrs { span : default_variant . ident . span , first : first . span , first_rest : rest [0] . span , rest : rest . iter () . map (| attr | attr . span) . collect :: < Vec < _ > > () . into () , only_one : rest . len () == 1 , sugg , }) ; return Err (guar) ; } } ; if ! attr . is_word () { let guar = cx . dcx () . emit_err (errors :: DefaultHasArg { span : attr . span }) ; return Err (guar) ; } Ok (()) }
    };
}

validate_default_attribute!()