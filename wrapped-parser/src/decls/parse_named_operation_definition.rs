macro_rules! deps {
    () => {
        Rule!();
        OperationDefinition!();
        Result!();
        PositionCalculator!();
        OperationDefinitionItem!();
    };
}

macro_rules! parse_named_operation_definition {
    () => {
        deps!();
        fn parse_named_operation_definition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < OperationDefinitionItem > { debug_assert_eq ! (pair . as_rule () , Rule :: named_operation_definition) ; let mut pairs = pair . into_inner () ; let ty = parse_operation_type (pairs . next () . unwrap () , pc) ? ; let name = parse_if_rule (& mut pairs , Rule :: name , | pair | parse_name (pair , pc)) ? ; let variable_definitions = parse_if_rule (& mut pairs , Rule :: variable_definitions , | pair | { parse_variable_definitions (pair , pc) }) ? ; let directives = parse_opt_directives (& mut pairs , pc) ? ; let selection_set = parse_selection_set (pairs . next () . unwrap () , pc , MAX_RECURSION_DEPTH) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok (OperationDefinitionItem { name , definition : OperationDefinition { ty : ty . node , variable_definitions : variable_definitions . unwrap_or_default () , directives , selection_set , } , }) }
    };
}

parse_named_operation_definition!()