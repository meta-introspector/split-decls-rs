macro_rules! deps {
    () => {
        Item!();
        Name!();
    };
}

macro_rules! derive_parser {
    () => {
        deps!();
        pub (crate) fn derive_parser (input : & DeriveInput) -> Result < TokenStream , syn :: Error > { let ident = & input . ident ; let pkg_name = std :: env :: var ("CARGO_PKG_NAME") . ok () . unwrap_or_default () ; match input . data { Data :: Struct (DataStruct { fields : Fields :: Named (ref fields) , .. }) => { let name = Name :: Assigned (quote ! (# pkg_name)) ; let item = Item :: from_args_struct (input , name) ? ; let fields = collect_args_fields (& item , fields) ? ; gen_for_struct (& item , ident , & input . generics , & fields) } Data :: Struct (DataStruct { fields : Fields :: Unit , .. }) => { let name = Name :: Assigned (quote ! (# pkg_name)) ; let item = Item :: from_args_struct (input , name) ? ; let fields = Punctuated :: < Field , Comma > :: new () ; let fields = fields . iter () . map (| field | { let item = Item :: from_args_field (field , item . casing () , item . env_casing ()) ? ; Ok ((field , item)) }) . collect :: < Result < Vec < _ > , syn :: Error > > () ? ; gen_for_struct (& item , ident , & input . generics , & fields) } Data :: Enum (ref e) => { let name = Name :: Assigned (quote ! (# pkg_name)) ; let item = Item :: from_subcommand_enum (input , name) ? ; let variants = e . variants . iter () . map (| variant | { let item = Item :: from_subcommand_variant (variant , item . casing () , item . env_casing ()) ? ; Ok ((variant , item)) }) . collect :: < Result < Vec < _ > , syn :: Error > > () ? ; gen_for_enum (& item , ident , & input . generics , & variants) } _ => abort_call_site ! ("`#[derive(Parser)]` only supports non-tuple structs and enums") , } }
    };
}

derive_parser!()