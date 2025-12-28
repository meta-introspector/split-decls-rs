macro_rules! deps {
    () => {
        DirectiveLocation!();
        FragmentSpread!();
        Positioned!();
        FragmentDefinition!();
        PositionCalculator!();
        VariableDefinition!();
        Result!();
        DirectiveDefinition!();
        Field!();
        Rule!();
        InlineFragment!();
        FieldDefinition!();
    };
}

macro_rules! parse_directive_definition {
    () => {
        deps!();
        fn parse_directive_definition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < DirectiveDefinition > > { debug_assert_eq ! (pair . as_rule () , Rule :: directive_definition) ; let pos = pc . step (& pair) ; let mut pairs = pair . into_inner () ; let description = parse_if_rule (& mut pairs , Rule :: string , | pair | parse_string (pair , pc)) ? ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let arguments = parse_if_rule (& mut pairs , Rule :: arguments_definition , | pair | { debug_assert_eq ! (pair . as_rule () , Rule :: arguments_definition) ; pair . into_inner () . map (| pair | parse_input_value_definition (pair , pc)) . collect () }) ? . unwrap_or_default () ; let is_repeatable = parse_if_rule (& mut pairs , Rule :: repeatable , | pair | { debug_assert_eq ! (pair . as_rule () , Rule :: repeatable) ; Ok (()) }) . unwrap_or_default () . is_some () ; let locations = { let pair = pairs . next () . unwrap () ; debug_assert_eq ! (pair . as_rule () , Rule :: directive_locations) ; pair . into_inner () . map (| pair | { let pos = pc . step (& pair) ; debug_assert_eq ! (pair . as_rule () , Rule :: directive_location) ; Positioned :: new (match pair . as_str () { "QUERY" => DirectiveLocation :: Query , "MUTATION" => DirectiveLocation :: Mutation , "SUBSCRIPTION" => DirectiveLocation :: Subscription , "FIELD" => DirectiveLocation :: Field , "FRAGMENT_DEFINITION" => DirectiveLocation :: FragmentDefinition , "FRAGMENT_SPREAD" => DirectiveLocation :: FragmentSpread , "INLINE_FRAGMENT" => DirectiveLocation :: InlineFragment , "VARIABLE_DEFINITION" => DirectiveLocation :: VariableDefinition , "SCHEMA" => DirectiveLocation :: Schema , "SCALAR" => DirectiveLocation :: Scalar , "OBJECT" => DirectiveLocation :: Object , "FIELD_DEFINITION" => DirectiveLocation :: FieldDefinition , "ARGUMENT_DEFINITION" => DirectiveLocation :: ArgumentDefinition , "INTERFACE" => DirectiveLocation :: Interface , "UNION" => DirectiveLocation :: Union , "ENUM" => DirectiveLocation :: Enum , "ENUM_VALUE" => DirectiveLocation :: EnumValue , "INPUT_OBJECT" => DirectiveLocation :: InputObject , "INPUT_FIELD_DEFINITION" => DirectiveLocation :: InputFieldDefinition , _ => unreachable ! () , } , pos ,) }) . collect () } ; debug_assert_eq ! (pairs . next () , None) ; Ok (Positioned :: new (DirectiveDefinition { description , name , arguments , is_repeatable , locations , } , pos ,)) }
    };
}

parse_directive_definition!();