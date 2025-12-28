macro_rules! deps {
    () => {
        Name!();
        Item!();
    };
}

macro_rules! derive_subcommand {
    () => {
        deps!();
        pub (crate) fn derive_subcommand (input : & DeriveInput) -> Result < TokenStream , syn :: Error > { let ident = & input . ident ; match input . data { Data :: Enum (ref e) => { let name = Name :: Derived (ident . clone ()) ; let item = Item :: from_subcommand_enum (input , name) ? ; let variants = e . variants . iter () . map (| variant | { let item = Item :: from_subcommand_variant (variant , item . casing () , item . env_casing ()) ? ; Ok ((variant , item)) }) . collect :: < Result < Vec < _ > , syn :: Error > > () ? ; gen_for_enum (& item , ident , & input . generics , & variants) } _ => abort_call_site ! ("`#[derive(Subcommand)]` only supports enums") , } }
    };
}

derive_subcommand!();