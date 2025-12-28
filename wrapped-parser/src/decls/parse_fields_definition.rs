macro_rules! deps {
    () => {
        Result!();
        PositionCalculator!();
        Rule!();
        Positioned!();
        FieldDefinition!();
    };
}

macro_rules! parse_fields_definition {
    () => {
        deps!();
        fn parse_fields_definition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < FieldDefinition > > > { debug_assert_eq ! (pair . as_rule () , Rule :: fields_definition) ; pair . into_inner () . map (| pair | parse_field_definition (pair , pc)) . collect () }
    };
}

parse_fields_definition!();