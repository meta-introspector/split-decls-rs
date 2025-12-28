macro_rules! deps {
    () => {
        IncorrectReprFormatGenericCause!();
        MetaItemParser!();
        ArgParser!();
        InvalidReprAlignNeedArg!();
        InvalidReprHintNoParen!();
        AlignKind!();
        IncorrectReprFormatGeneric!();
        Stage!();
        AcceptContext!();
        UnrecognizedReprHint!();
        InvalidReprHintNoValue!();
    };
}

macro_rules! parse_repr {
    () => {
        deps!();
        fn parse_repr < S : Stage > (cx : & AcceptContext < '_ , '_ , S > , param : & MetaItemParser < '_ > ,) -> Option < ReprAttr > { use ReprAttr :: * ; let (name , ident_span) = if let Some (ident) = param . path () . word () { (Some (ident . name) , ident . span) } else { (None , DUMMY_SP) } ; let args = param . args () ; match (name , args) { (Some (sym :: align) , ArgParser :: NoArgs) => { cx . emit_err (session_diagnostics :: InvalidReprAlignNeedArg { span : ident_span }) ; None } (Some (sym :: align) , ArgParser :: List (l)) => { parse_repr_align (cx , l , param . span () , AlignKind :: Align) } (Some (sym :: packed) , ArgParser :: NoArgs) => Some (ReprPacked (Align :: ONE)) , (Some (sym :: packed) , ArgParser :: List (l)) => { parse_repr_align (cx , l , param . span () , AlignKind :: Packed) } (Some (name @ sym :: align | name @ sym :: packed) , ArgParser :: NameValue (l)) => { cx . emit_err (session_diagnostics :: IncorrectReprFormatGeneric { span : param . span () , repr_arg : name , cause : IncorrectReprFormatGenericCause :: from_lit_kind (param . span () , & l . value_as_lit () . kind , name ,) , }) ; None } (Some (sym :: Rust) , ArgParser :: NoArgs) => Some (ReprRust) , (Some (sym :: C) , ArgParser :: NoArgs) => Some (ReprC) , (Some (sym :: simd) , ArgParser :: NoArgs) => Some (ReprSimd) , (Some (sym :: transparent) , ArgParser :: NoArgs) => Some (ReprTransparent) , (Some (name @ int_pat ! ()) , ArgParser :: NoArgs) => { Some (ReprInt (int_type_of_word (name) . unwrap ())) } (Some (name @ sym :: Rust | name @ sym :: C | name @ sym :: simd | name @ sym :: transparent | name @ int_pat ! () ,) , ArgParser :: NameValue (_) ,) => { cx . emit_err (session_diagnostics :: InvalidReprHintNoValue { span : param . span () , name }) ; None } (Some (name @ sym :: Rust | name @ sym :: C | name @ sym :: simd | name @ sym :: transparent | name @ int_pat ! () ,) , ArgParser :: List (_) ,) => { cx . emit_err (session_diagnostics :: InvalidReprHintNoParen { span : param . span () , name }) ; None } _ => { cx . emit_err (session_diagnostics :: UnrecognizedReprHint { span : param . span () }) ; None } } }
    };
}

parse_repr!()