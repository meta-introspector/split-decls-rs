macro_rules! deps {
    () => {
        PositionCalculator!();
        Result!();
        FieldDefinition!();
        Rule!();
        Positioned!();
    };
}

macro_rules! parse_field_definition {
    () => {
        deps!();
        fn parse_field_definition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < FieldDefinition > > { debug_assert_eq ! (pair . as_rule () , Rule :: field_definition) ; let pos = pc . step (& pair) ; let mut pairs = pair . into_inner () ; let description = parse_if_rule (& mut pairs , Rule :: string , | pair | parse_string (pair , pc)) ? ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let arguments = parse_if_rule (& mut pairs , Rule :: arguments_definition , | pair | { parse_arguments_definition (pair , pc) }) ? . unwrap_or_default () ; let ty = parse_type (pairs . next () . unwrap () , pc) ? ; let directives = parse_opt_const_directives (& mut pairs , pc) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok (Positioned :: new (FieldDefinition { description , name , arguments , ty , directives , } , pos ,)) }
    };
}

parse_field_definition!()