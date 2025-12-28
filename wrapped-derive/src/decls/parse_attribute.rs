macro_rules! deps {
    () => {
        FieldConstructor!();
    };
}

macro_rules! parse_attribute {
    () => {
        deps!();
        fn parse_attribute (attr : & Attribute) -> Result < FieldConstructor > { if let Meta :: List (ref meta_list) = attr . meta { parse_attribute_internals (meta_list) } else { let msg = format ! ("#[{ARBITRARY_ATTRIBUTE_NAME}] must contain a group") ; Err (syn :: Error :: new (attr . span () , msg)) } }
    };
}

parse_attribute!();