macro_rules! deps {
    () => {
        Item!();
        Name!();
    };
}

macro_rules! derive_args {
    () => {
        deps!();
        pub (crate) fn derive_args (input : & DeriveInput) -> Result < TokenStream , syn :: Error > { let ident = & input . ident ; match input . data { Data :: Struct (DataStruct { fields : Fields :: Named (ref fields) , .. }) => { let name = Name :: Derived (ident . clone ()) ; let item = Item :: from_args_struct (input , name) ? ; let fields = collect_args_fields (& item , fields) ? ; gen_for_struct (& item , ident , & input . generics , & fields) } Data :: Struct (DataStruct { fields : Fields :: Unit , .. }) => { let name = Name :: Derived (ident . clone ()) ; let item = Item :: from_args_struct (input , name) ? ; let fields = Punctuated :: < Field , Comma > :: new () ; let fields = fields . iter () . map (| field | { let item = Item :: from_args_field (field , item . casing () , item . env_casing ()) ? ; Ok ((field , item)) }) . collect :: < Result < Vec < _ > , syn :: Error > > () ? ; gen_for_struct (& item , ident , & input . generics , & fields) } _ => abort_call_site ! ("`#[derive(Args)]` only supports non-tuple structs") , } }
    };
}

derive_args!()