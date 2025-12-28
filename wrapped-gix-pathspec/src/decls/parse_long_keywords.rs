macro_rules! deps {
    () => {
        Error!();
        Pattern!();
        SearchMode!();
    };
}

macro_rules! parse_long_keywords {
    () => {
        deps!();
        fn parse_long_keywords (input : & [u8] , p : & mut Pattern , cursor : & mut usize) -> Result < () , Error > { let end = input . find (")") . ok_or (Error :: MissingClosingParenthesis) ? ; let input = & input [* cursor .. end] ; * cursor = end + 1 ; if input . is_empty () { return Ok (()) ; } split_on_non_escaped_char (input , b',' , | keyword | { let attr_prefix = b"attr:" ; match keyword { b"attr" => { } b"top" => p . signature |= MagicSignature :: TOP , b"icase" => p . signature |= MagicSignature :: ICASE , b"exclude" => p . signature |= MagicSignature :: EXCLUDE , b"literal" => match p . search_mode { SearchMode :: PathAwareGlob => return Err (Error :: IncompatibleSearchModes) , _ => p . search_mode = SearchMode :: Literal , } , b"glob" => match p . search_mode { SearchMode :: Literal => return Err (Error :: IncompatibleSearchModes) , _ => p . search_mode = SearchMode :: PathAwareGlob , } , _ if keyword . starts_with (attr_prefix) => { if p . attributes . is_empty () { p . attributes = parse_attributes (& keyword [attr_prefix . len () ..]) ? ; } else { return Err (Error :: MultipleAttributeSpecifications) ; } } _ => { return Err (Error :: InvalidKeyword { keyword : BString :: from (keyword) , }) ; } } Ok (()) }) }
    };
}

parse_long_keywords!()