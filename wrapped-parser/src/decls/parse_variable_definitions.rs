macro_rules! deps {
    () => {
        PositionCalculator!();
        Positioned!();
        VariableDefinition!();
        Result!();
        Rule!();
    };
}

macro_rules! parse_variable_definitions {
    () => {
        deps!();
        fn parse_variable_definitions (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < VariableDefinition > > > { debug_assert_eq ! (pair . as_rule () , Rule :: variable_definitions) ; pair . into_inner () . map (| pair | parse_variable_definition (pair , pc)) . collect () }
    };
}

parse_variable_definitions!()