macro_rules! deps {
    () => {
        Positioned!();
        EnumValueDefinition!();
        Result!();
        TypeDefinition!();
        Rule!();
        InputObjectType!();
        InterfaceType!();
        PositionCalculator!();
        TypeKind!();
        EnumType!();
        UnionType!();
        ObjectType!();
    };
}

macro_rules! parse_type_definition {
    () => {
        deps!();
        fn parse_type_definition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < TypeDefinition > > { debug_assert_eq ! (pair . as_rule () , Rule :: type_definition) ; let pos = pc . step (& pair) ; let pair = exactly_one (pair . into_inner ()) ; let rule = pair . as_rule () ; let mut pairs = pair . into_inner () ; let description = parse_if_rule (& mut pairs , Rule :: string , | pair | parse_string (pair , pc)) ? ; let extend = next_if_rule (& mut pairs , Rule :: extend) . is_some () ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let (directives , kind) = match rule { Rule :: scalar_type => { let directives = parse_opt_const_directives (& mut pairs , pc) ? ; (directives , TypeKind :: Scalar) } Rule :: object_type => { let implements = parse_if_rule (& mut pairs , Rule :: implements_interfaces , | pair | { debug_assert_eq ! (pair . as_rule () , Rule :: implements_interfaces) ; pair . into_inner () . map (| pair | parse_name (pair , pc)) . collect :: < Result < _ > > () }) ? ; let directives = parse_opt_const_directives (& mut pairs , pc) ? ; let fields = parse_if_rule (& mut pairs , Rule :: fields_definition , | pair | { parse_fields_definition (pair , pc) }) ? . unwrap_or_default () ; (directives , TypeKind :: Object (ObjectType { implements : implements . unwrap_or_default () , fields , }) ,) } Rule :: interface_type => { let implements = parse_if_rule (& mut pairs , Rule :: implements_interfaces , | pair | { debug_assert_eq ! (pair . as_rule () , Rule :: implements_interfaces) ; pair . into_inner () . map (| pair | parse_name (pair , pc)) . collect :: < Result < _ > > () }) ? ; let directives = parse_opt_const_directives (& mut pairs , pc) ? ; let fields = parse_if_rule (& mut pairs , Rule :: fields_definition , | pair | { parse_fields_definition (pair , pc) }) ? . unwrap_or_default () ; (directives , TypeKind :: Interface (InterfaceType { implements : implements . unwrap_or_default () , fields , }) ,) } Rule :: union_type => { let directives = parse_opt_const_directives (& mut pairs , pc) ? ; let members = parse_if_rule (& mut pairs , Rule :: union_member_types , | pair | { debug_assert_eq ! (pair . as_rule () , Rule :: union_member_types) ; pair . into_inner () . map (| pair | parse_name (pair , pc)) . collect () }) ? . unwrap_or_default () ; (directives , TypeKind :: Union (UnionType { members })) } Rule :: enum_type => { let directives = parse_opt_const_directives (& mut pairs , pc) ? ; let values = parse_if_rule (& mut pairs , Rule :: enum_values , | pair | { debug_assert_eq ! (pair . as_rule () , Rule :: enum_values) ; pair . into_inner () . map (| pair | { debug_assert_eq ! (pair . as_rule () , Rule :: enum_value_definition) ; let pos = pc . step (& pair) ; let mut pairs = pair . into_inner () ; let description = parse_if_rule (& mut pairs , Rule :: string , | pair | parse_string (pair , pc)) ? ; let value = parse_enum_value (pairs . next () . unwrap () , pc) ? ; let directives = parse_opt_const_directives (& mut pairs , pc) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok (Positioned :: new (EnumValueDefinition { description , value , directives , } , pos ,)) }) . collect () }) ? . unwrap_or_default () ; (directives , TypeKind :: Enum (EnumType { values })) } Rule :: input_object_type => { let directives = parse_opt_const_directives (& mut pairs , pc) ? ; let fields = parse_if_rule (& mut pairs , Rule :: input_fields_definition , | pair | { debug_assert_eq ! (pair . as_rule () , Rule :: input_fields_definition) ; pair . into_inner () . map (| pair | parse_input_value_definition (pair , pc)) . collect () }) ? . unwrap_or_default () ; (directives , TypeKind :: InputObject (InputObjectType { fields }) ,) } _ => unreachable ! () , } ; debug_assert_eq ! (pairs . next () , None) ; Ok (Positioned :: new (TypeDefinition { extend , description , name , directives , kind , } , pos ,)) }
    };
}

parse_type_definition!();