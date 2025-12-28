macro_rules! deps {
    () => {
        GeneratorResult!();
    };
}

macro_rules! generate_fields_enum {
    () => {
        deps!();
        fn generate_fields_enum (crate_name : & proc_macro2 :: TokenStream , fields : Vec < (String , Ident , Vec < Attribute >) > ,) -> GeneratorResult < proc_macro2 :: TokenStream > { if fields . is_empty () { return Ok (quote ! ()) ; } let enum_variants = fields . iter () . map (| (_ , field_ident , cfg_attrs) | { quote ! { # (# cfg_attrs) * # field_ident } }) ; let matches = fields . iter () . map (| (field_name , field_ident , cfg_attrs) | { quote ! { # (# cfg_attrs) * # field_name => :: std :: option :: Option :: Some (__FieldIdent ::# field_ident) , } }) ; Ok (quote ! { # [allow (non_camel_case_types)] # [doc (hidden)] enum __FieldIdent { # (# enum_variants ,) * } impl __FieldIdent { fn from_name (__name : &# crate_name :: Name) -> :: std :: option :: Option < __FieldIdent > { match __name . as_str () { # (# matches) * _ => :: std :: option :: Option :: None } } } }) }
    };
}

generate_fields_enum!()