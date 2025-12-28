macro_rules! gen_constructor_for_field {
    () => {
        fn gen_constructor_for_field (field : & Field) -> Result < TokenStream > { let ctor = match determine_field_constructor (field) ? { FieldConstructor :: Default => quote ! (:: core :: default :: Default :: default ()) , FieldConstructor :: Arbitrary => quote ! (arbitrary :: Arbitrary :: arbitrary (u) ?) , FieldConstructor :: With (function_or_closure) => quote ! ((# function_or_closure) (u) ?) , FieldConstructor :: Value (value) => quote ! (# value) , } ; Ok (ctor) }
    };
}

gen_constructor_for_field!()