macro_rules! deps {
    () => {
        Positioned!();
        VariableDefinition!();
        PositionCalculator!();
        Result!();
        Rule!();
    };
}

macro_rules! parse_variable_definition {
    () => {
        deps!();
        fn parse_variable_definition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < VariableDefinition > > { debug_assert_eq ! (pair . as_rule () , Rule :: variable_definition) ; let pos = pc . step (& pair) ; let mut pairs = pair . into_inner () ; let variable = parse_variable (pairs . next () . unwrap () , pc) ? ; let var_type = parse_type (pairs . next () . unwrap () , pc) ? ; let directives = parse_opt_directives (& mut pairs , pc) ? ; let default_value = parse_if_rule (& mut pairs , Rule :: default_value , | pair | { parse_default_value (pair , pc) }) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok (Positioned :: new (VariableDefinition { name : variable , var_type , directives , default_value , } , pos ,)) }
    };
}

parse_variable_definition!();