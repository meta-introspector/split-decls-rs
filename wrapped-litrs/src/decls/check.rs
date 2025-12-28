macro_rules! deps {
    () => {
        StringLit!();
        Literal!();
    };
}

macro_rules! check {
    () => {
        deps!();
        macro_rules ! check { ($ lit : literal , $ has_escapes : expr , $ num_hashes : expr) => { check ! ($ lit , stringify ! ($ lit) , $ has_escapes , $ num_hashes , "") } ; ($ lit : literal , $ input : expr , $ has_escapes : expr , $ num_hashes : expr , $ suffix : literal) => { let input = $ input ; let expected = StringLit { raw : input , value : if $ has_escapes { Some ($ lit . to_string ()) } else { None } , num_hashes : $ num_hashes , start_suffix : input . len () - $ suffix . len () , } ; assert_parse_ok_eq (input , StringLit :: parse (input) , expected . clone () , "StringLit::parse") ; assert_parse_ok_eq (input , Literal :: parse (input) , Literal :: String (expected . clone ()) , "Literal::parse") ; let lit = StringLit :: parse (input) . unwrap () ; assert_eq ! (lit . value () , $ lit) ; assert_eq ! (lit . suffix () , $ suffix) ; assert_eq ! (lit . into_value () , $ lit) ; assert_roundtrip (expected . into_owned () , input) ; } ; }
    };
}

check!();