macro_rules! deps {
    () => {
        Positioned!();
        PositionCalculator!();
        Result!();
        Rule!();
    };
}

macro_rules! parse_alias {
    () => {
        deps!();
        fn parse_alias (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Name > > { debug_assert_eq ! (pair . as_rule () , Rule :: alias) ; parse_name (exactly_one (pair . into_inner ()) , pc) }
    };
}

parse_alias!();