macro_rules! deps {
    () => {
        Name!();
        Item!();
    };
}

macro_rules! derive_value_enum {
    () => {
        deps!();
        pub (crate) fn derive_value_enum (input : & DeriveInput) -> Result < TokenStream , syn :: Error > { let ident = & input . ident ; match input . data { Data :: Enum (ref e) => { let name = Name :: Derived (ident . clone ()) ; let item = Item :: from_value_enum (input , name) ? ; let mut variants = Vec :: new () ; for variant in & e . variants { let item = Item :: from_value_enum_variant (variant , item . casing () , item . env_casing ()) ? ; variants . push ((variant , item)) ; } gen_for_enum (& item , ident , & variants) } _ => abort_call_site ! ("`#[derive(ValueEnum)]` only supports enums") , } }
    };
}

derive_value_enum!();