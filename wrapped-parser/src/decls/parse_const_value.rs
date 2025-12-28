macro_rules! deps {
    () => {
        Rule!();
        PositionCalculator!();
        Positioned!();
        Result!();
    };
}

macro_rules! parse_const_value {
    () => {
        deps!();
        fn parse_const_value (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < ConstValue > > { debug_assert_eq ! (pair . as_rule () , Rule :: const_value) ; let pos = pc . step (& pair) ; let pair = exactly_one (pair . into_inner ()) ; Ok (Positioned :: new (match pair . as_rule () { Rule :: number => ConstValue :: Number (parse_number (pair , pc) ? . node) , Rule :: string => ConstValue :: String (parse_string (pair , pc) ? . node) , Rule :: boolean => ConstValue :: Boolean (parse_boolean (pair , pc) ? . node) , Rule :: null => ConstValue :: Null , Rule :: enum_value => ConstValue :: Enum (parse_enum_value (pair , pc) ? . node) , Rule :: const_list => ConstValue :: List (pair . into_inner () . map (| pair | Ok (parse_const_value (pair , pc) ? . node)) . collect :: < Result < _ > > () ? ,) , Rule :: const_object => ConstValue :: Object (pair . into_inner () . map (| pair | { debug_assert_eq ! (pair . as_rule () , Rule :: const_object_field) ; let mut pairs = pair . into_inner () ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let value = parse_const_value (pairs . next () . unwrap () , pc) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok ((name . node , value . node)) }) . collect :: < Result < _ > > () ? ,) , _ => unreachable ! () , } , pos ,)) }
    };
}

parse_const_value!();