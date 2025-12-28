macro_rules! deps {
    () => {
        Rule!();
        Positioned!();
        PositionCalculator!();
        Result!();
    };
}

macro_rules! parse_name {
    () => {
        deps!();
        fn parse_name (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Name > > { debug_assert_eq ! (pair . as_rule () , Rule :: name) ; Ok (Positioned :: new (Name :: new (pair . as_str ()) , pc . step (& pair))) }
    };
}

parse_name!();