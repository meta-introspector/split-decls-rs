macro_rules! deps {
    () => {
        Rule!();
        DefinitionItem!();
        Result!();
        PositionCalculator!();
    };
}

macro_rules! parse_definition_item {
    () => {
        deps!();
        fn parse_definition_item (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < DefinitionItem > { debug_assert_eq ! (pair . as_rule () , Rule :: executable_definition) ; let pair = exactly_one (pair . into_inner ()) ; Ok (match pair . as_rule () { Rule :: operation_definition => { DefinitionItem :: Operation (parse_operation_definition_item (pair , pc) ?) } Rule :: fragment_definition => { DefinitionItem :: Fragment (parse_fragment_definition_item (pair , pc) ?) } _ => unreachable ! () , }) }
    };
}

parse_definition_item!()