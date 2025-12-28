macro_rules! deps {
    () => {
        MetaBadDelimSugg!();
        MetaBadDelim!();
    };
}

macro_rules! check_meta_bad_delim {
    () => {
        deps!();
        fn check_meta_bad_delim (psess : & ParseSess , span : DelimSpan , delim : Delimiter) { if let Delimiter :: Parenthesis = delim { return ; } psess . dcx () . emit_err (errors :: MetaBadDelim { span : span . entire () , sugg : errors :: MetaBadDelimSugg { open : span . open , close : span . close } , }) ; }
    };
}

check_meta_bad_delim!()