macro_rules! deps {
    () => {
        PositionCalculator!();
        Result!();
        Positioned!();
        Rule!();
        InputValueDefinition!();
    };
}

macro_rules! parse_input_value_definition {
    () => {
        deps!();
        fn parse_input_value_definition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < InputValueDefinition > > { debug_assert_eq ! (pair . as_rule () , Rule :: input_value_definition) ; let pos = pc . step (& pair) ; let mut pairs = pair . into_inner () ; let description = parse_if_rule (& mut pairs , Rule :: string , | pair | parse_string (pair , pc)) ? ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let ty = parse_type (pairs . next () . unwrap () , pc) ? ; let default_value = parse_if_rule (& mut pairs , Rule :: default_value , | pair | { parse_default_value (pair , pc) }) ? ; let directives = parse_opt_const_directives (& mut pairs , pc) ? ; Ok (Positioned :: new (InputValueDefinition { description , name , ty , default_value , directives , } , pos ,)) }
    };
}

parse_input_value_definition!();