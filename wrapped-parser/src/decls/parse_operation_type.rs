macro_rules! deps {
    () => {
        Result!();
        OperationType!();
        PositionCalculator!();
        Rule!();
        Positioned!();
    };
}

macro_rules! parse_operation_type {
    () => {
        deps!();
        fn parse_operation_type (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < OperationType > > { debug_assert_eq ! (pair . as_rule () , Rule :: operation_type) ; let pos = pc . step (& pair) ; Ok (Positioned :: new (match pair . as_str () { "query" => OperationType :: Query , "mutation" => OperationType :: Mutation , "subscription" => OperationType :: Subscription , _ => unreachable ! () , } , pos ,)) }
    };
}

parse_operation_type!()