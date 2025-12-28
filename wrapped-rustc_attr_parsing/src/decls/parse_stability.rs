macro_rules! deps {
    () => {
        ArgParser!();
        InvalidSince!();
        Stage!();
        MissingFeature!();
        UnsupportedLiteral!();
        UnknownMetaItem!();
        UnsupportedLiteralReason!();
        NonIdentFeature!();
        MissingSince!();
        AcceptContext!();
    };
}

macro_rules! parse_stability {
    () => {
        deps!();
        # [doc = " Read the content of a `stable`/`rustc_const_stable` attribute, and return the feature name and"] # [doc = " its stability information."] pub (crate) fn parse_stability < S : Stage > (cx : & AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ > ,) -> Option < (Symbol , StabilityLevel) > { let mut feature = None ; let mut since = None ; let ArgParser :: List (list) = args else { cx . expected_list (cx . attr_span) ; return None ; } ; for param in list . mixed () { let param_span = param . span () ; let Some (param) = param . meta_item () else { cx . emit_err (session_diagnostics :: UnsupportedLiteral { span : param_span , reason : UnsupportedLiteralReason :: Generic , is_bytestr : false , start_point_span : cx . sess () . source_map () . start_point (param_span) , }) ; return None ; } ; let word = param . path () . word () ; match word . map (| i | i . name) { Some (sym :: feature) => { insert_value_into_option_or_error (cx , & param , & mut feature , word . unwrap ()) ? } Some (sym :: since) => { insert_value_into_option_or_error (cx , & param , & mut since , word . unwrap ()) ? } _ => { cx . emit_err (session_diagnostics :: UnknownMetaItem { span : param_span , item : param . path () . to_string () , expected : & ["feature" , "since"] , }) ; return None ; } } } let feature = match feature { Some (feature) if rustc_lexer :: is_ident (feature . as_str ()) => Ok (feature) , Some (_bad_feature) => { Err (cx . emit_err (session_diagnostics :: NonIdentFeature { span : cx . attr_span })) } None => Err (cx . emit_err (session_diagnostics :: MissingFeature { span : cx . attr_span })) , } ; let since = if let Some (since) = since { if since . as_str () == VERSION_PLACEHOLDER { StableSince :: Current } else if let Some (version) = parse_version (since) { StableSince :: Version (version) } else { let err = cx . emit_err (session_diagnostics :: InvalidSince { span : cx . attr_span }) ; StableSince :: Err (err) } } else { let err = cx . emit_err (session_diagnostics :: MissingSince { span : cx . attr_span }) ; StableSince :: Err (err) } ; match feature { Ok (feature) => { let level = StabilityLevel :: Stable { since , allowed_through_unstable_modules : None } ; Some ((feature , level)) } Err (ErrorGuaranteed { .. }) => None , } }
    };
}

parse_stability!()