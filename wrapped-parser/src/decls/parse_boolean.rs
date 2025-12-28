macro_rules! deps {
    () => {
        Positioned!();
        PositionCalculator!();
        Rule!();
        Result!();
    };
}

macro_rules! parse_boolean {
    () => {
        deps!();
        fn parse_boolean (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < bool > > { debug_assert_eq ! (pair . as_rule () , Rule :: boolean) ; let pos = pc . step (& pair) ; Ok (Positioned :: new (match pair . as_str () { "true" => true , "false" => false , _ => unreachable ! () , } , pos ,)) }
    };
}

parse_boolean!();