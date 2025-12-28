macro_rules! deps {
    () => {
        Rule!();
        Result!();
        Positioned!();
        PositionCalculator!();
    };
}

macro_rules! parse_arguments {
    () => {
        deps!();
        fn parse_arguments (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < (Positioned < Name > , Positioned < Value >) > > { debug_assert_eq ! (pair . as_rule () , Rule :: arguments) ; pair . into_inner () . map (| pair | { debug_assert_eq ! (pair . as_rule () , Rule :: argument) ; let mut pairs = pair . into_inner () ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let value = parse_value (pairs . next () . unwrap () , pc) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok ((name , value)) }) . collect () }
    };
}

parse_arguments!()