macro_rules! deps {
    () => {
        Rule!();
        Type!();
        Result!();
        PositionCalculator!();
        Positioned!();
    };
}

macro_rules! parse_type {
    () => {
        deps!();
        fn parse_type (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Type > > { debug_assert_eq ! (pair . as_rule () , Rule :: type_) ; Ok (Positioned :: new (Type :: new (pair . as_str ()) . unwrap () , pc . step (& pair) ,)) }
    };
}

parse_type!()