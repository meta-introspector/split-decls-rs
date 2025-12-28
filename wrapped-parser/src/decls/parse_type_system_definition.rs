macro_rules! deps {
    () => {
        Type!();
        PositionCalculator!();
        TypeSystemDefinition!();
        Rule!();
        Result!();
        Directive!();
    };
}

macro_rules! parse_type_system_definition {
    () => {
        deps!();
        fn parse_type_system_definition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < TypeSystemDefinition > { debug_assert_eq ! (pair . as_rule () , Rule :: type_system_definition) ; let pair = exactly_one (pair . into_inner ()) ; Ok (match pair . as_rule () { Rule :: schema_definition => TypeSystemDefinition :: Schema (parse_schema_definition (pair , pc) ?) , Rule :: type_definition => TypeSystemDefinition :: Type (parse_type_definition (pair , pc) ?) , Rule :: directive_definition => { TypeSystemDefinition :: Directive (parse_directive_definition (pair , pc) ?) } _ => unreachable ! () , }) }
    };
}

parse_type_system_definition!();