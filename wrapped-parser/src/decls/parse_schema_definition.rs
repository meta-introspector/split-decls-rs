macro_rules! deps {
    () => {
        Error!();
        Positioned!();
        SchemaDefinition!();
        Rule!();
        Result!();
        PositionCalculator!();
        OperationType!();
    };
}

macro_rules! parse_schema_definition {
    () => {
        deps!();
        fn parse_schema_definition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < SchemaDefinition > > { debug_assert_eq ! (pair . as_rule () , Rule :: schema_definition) ; let pos = pc . step (& pair) ; let mut pairs = pair . into_inner () ; let extend = next_if_rule (& mut pairs , Rule :: extend) . is_some () ; let directives = parse_opt_const_directives (& mut pairs , pc) ? ; let mut query = None ; let mut mutation = None ; let mut subscription = None ; for pair in pairs { debug_assert_eq ! (pair . as_rule () , Rule :: operation_type_definition) ; let mut pairs = pair . into_inner () ; let operation_type = parse_operation_type (pairs . next () . unwrap () , pc) ? ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; match operation_type . node { OperationType :: Query if query . is_none () => query = Some (name) , OperationType :: Mutation if mutation . is_none () => mutation = Some (name) , OperationType :: Subscription if subscription . is_none () => subscription = Some (name) , _ => { return Err (Error :: MultipleRoots { root : operation_type . node , schema : pos , pos : operation_type . pos , }) ; } } debug_assert_eq ! (pairs . next () , None) ; } if ! extend && query . is_none () { return Err (Error :: MissingQueryRoot { pos }) ; } Ok (Positioned :: new (SchemaDefinition { extend , directives , query , mutation , subscription , } , pos ,)) }
    };
}

parse_schema_definition!();