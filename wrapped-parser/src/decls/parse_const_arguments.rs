macro_rules! deps {
    () => {
        Rule!();
        Result!();
        PositionCalculator!();
        Positioned!();
    };
}

macro_rules! parse_const_arguments {
    () => {
        deps!();
        fn parse_const_arguments (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < (Positioned < Name > , Positioned < ConstValue >) > > { debug_assert_eq ! (pair . as_rule () , Rule :: const_arguments) ; pair . into_inner () . map (| pair | { debug_assert_eq ! (pair . as_rule () , Rule :: const_argument) ; let mut pairs = pair . into_inner () ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let value = parse_const_value (pairs . next () . unwrap () , pc) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok ((name , value)) }) . collect () }
    };
}

parse_const_arguments!()