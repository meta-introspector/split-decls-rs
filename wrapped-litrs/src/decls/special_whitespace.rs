macro_rules! deps {
    () => {
        Literal!();
        StringLit!();
    };
}

macro_rules! special_whitespace {
    () => {
        deps!();
        # [test] fn special_whitespace () { let strings = ["\n" , "\t" , "foo\tbar" , "🦊\n"] ; for & s in & strings { let input = format ! (r#""{}""# , s) ; let input_raw = format ! (r#"r"{}""# , s) ; for (input , num_hashes) in vec ! [(input , None) , (input_raw , Some (0))] { let expected = StringLit { raw : & * input , value : None , num_hashes , start_suffix : input . len () , } ; assert_parse_ok_eq (& input , StringLit :: parse (& * input) , expected . clone () , "StringLit::parse") ; assert_parse_ok_eq (& input , Literal :: parse (& * input) , Literal :: String (expected) , "Literal::parse") ; assert_eq ! (StringLit :: parse (&* input) . unwrap () . value () , s) ; assert_eq ! (StringLit :: parse (&* input) . unwrap () . into_value () , s) ; } } }
    };
}

special_whitespace!();