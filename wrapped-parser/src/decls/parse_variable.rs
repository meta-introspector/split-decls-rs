macro_rules! deps {
    () => {
        Positioned!();
        PositionCalculator!();
        Rule!();
        Result!();
    };
}

macro_rules! parse_variable {
    () => {
        deps!();
        fn parse_variable (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Name > > { debug_assert_eq ! (pair . as_rule () , Rule :: variable) ; parse_name (exactly_one (pair . into_inner ()) , pc) }
    };
}

parse_variable!()