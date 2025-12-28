macro_rules! deps {
    () => {
        InputValueDefinition!();
        Result!();
        Rule!();
        PositionCalculator!();
        Positioned!();
    };
}

macro_rules! parse_arguments_definition {
    () => {
        deps!();
        fn parse_arguments_definition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < InputValueDefinition > > > { debug_assert_eq ! (pair . as_rule () , Rule :: arguments_definition) ; pair . into_inner () . map (| pair | parse_input_value_definition (pair , pc)) . collect () }
    };
}

parse_arguments_definition!();