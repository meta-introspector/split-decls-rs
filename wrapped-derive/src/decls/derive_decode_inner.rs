macro_rules! deps {
    () => {
        DeriveStruct!();
        ContainerAttributes!();
        DeriveEnum!();
    };
}

macro_rules! derive_decode_inner {
    () => {
        deps!();
        fn derive_decode_inner (input : TokenStream) -> Result < TokenStream > { let parse = Parse :: new (input) ? ; let (mut generator , attributes , body) = parse . into_generator () ; let attributes = attributes . get_attribute :: < ContainerAttributes > () ? . unwrap_or_default () ; match body { Body :: Struct (body) => { derive_struct :: DeriveStruct { fields : body . fields , attributes , } . generate_decode (& mut generator) ? ; } Body :: Enum (body) => { derive_enum :: DeriveEnum { variants : body . variants , attributes , } . generate_decode (& mut generator) ? ; } } generator . export_to_file ("bincode" , "Decode") ; generator . finish () }
    };
}

derive_decode_inner!()