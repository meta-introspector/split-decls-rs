macro_rules! deps {
    () => {
        PositionCalculator!();
        Positioned!();
        Result!();
        Rule!();
        TypeCondition!();
    };
}

macro_rules! parse_type_condition {
    () => {
        deps!();
        fn parse_type_condition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < TypeCondition > > { debug_assert_eq ! (pair . as_rule () , Rule :: type_condition) ; let pos = pc . step (& pair) ; Ok (Positioned :: new (TypeCondition { on : parse_name (exactly_one (pair . into_inner ()) , pc) ? , } , pos ,)) }
    };
}

parse_type_condition!();