macro_rules! deps {
    () => {
        FieldConstructor!();
    };
}

macro_rules! determine_field_constructor {
    () => {
        deps!();
        pub fn determine_field_constructor (field : & Field) -> Result < FieldConstructor > { let opt_attr = fetch_attr_from_field (field) ? ; let ctor = match opt_attr { Some (attr) => parse_attribute (attr) ? , None => FieldConstructor :: Arbitrary , } ; Ok (ctor) }
    };
}

determine_field_constructor!();