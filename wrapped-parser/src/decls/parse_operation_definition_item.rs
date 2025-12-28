macro_rules! deps {
    () => {
        PositionCalculator!();
        Rule!();
        Positioned!();
        OperationDefinition!();
        OperationType!();
        Result!();
        OperationDefinitionItem!();
    };
}

macro_rules! parse_operation_definition_item {
    () => {
        deps!();
        fn parse_operation_definition_item (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < OperationDefinitionItem > > { debug_assert_eq ! (pair . as_rule () , Rule :: operation_definition) ; let pos = pc . step (& pair) ; let pair = exactly_one (pair . into_inner ()) ; Ok (Positioned :: new (match pair . as_rule () { Rule :: named_operation_definition => parse_named_operation_definition (pair , pc) ? , Rule :: selection_set => OperationDefinitionItem { name : None , definition : OperationDefinition { ty : OperationType :: Query , variable_definitions : Vec :: new () , directives : Vec :: new () , selection_set : parse_selection_set (pair , pc , MAX_RECURSION_DEPTH) ? , } , } , _ => unreachable ! () , } , pos ,)) }
    };
}

parse_operation_definition_item!()