macro_rules! deps {
    () => {
        Result!();
        Positioned!();
        Rule!();
        PositionCalculator!();
    };
}

macro_rules! parse_value {
    () => {
        deps!();
        fn parse_value (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Value > > { debug_assert_eq ! (pair . as_rule () , Rule :: value) ; let pos = pc . step (& pair) ; let pair = exactly_one (pair . into_inner ()) ; Ok (Positioned :: new (match pair . as_rule () { Rule :: variable => Value :: Variable (parse_variable (pair , pc) ? . node) , Rule :: number => Value :: Number (parse_number (pair , pc) ? . node) , Rule :: string => Value :: String (parse_string (pair , pc) ? . node) , Rule :: boolean => Value :: Boolean (parse_boolean (pair , pc) ? . node) , Rule :: null => Value :: Null , Rule :: enum_value => Value :: Enum (parse_enum_value (pair , pc) ? . node) , Rule :: list => Value :: List (pair . into_inner () . map (| pair | Ok (parse_value (pair , pc) ? . node)) . collect :: < Result < _ > > () ? ,) , Rule :: object => Value :: Object (pair . into_inner () . map (| pair | { debug_assert_eq ! (pair . as_rule () , Rule :: object_field) ; let mut pairs = pair . into_inner () ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let value = parse_value (pairs . next () . unwrap () , pc) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok ((name . node , value . node)) }) . collect :: < Result < _ > > () ? ,) , _ => unreachable ! () , } , pos ,)) }
    };
}

parse_value!()