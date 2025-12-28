macro_rules! deps {
    () => {
        Result!();
        Positioned!();
        Rule!();
        PositionCalculator!();
    };
}

macro_rules! parse_enum_value {
    () => {
        deps!();
        fn parse_enum_value (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Name > > { debug_assert_eq ! (pair . as_rule () , Rule :: enum_value) ; parse_name (exactly_one (pair . into_inner ()) , pc) }
    };
}

parse_enum_value!()