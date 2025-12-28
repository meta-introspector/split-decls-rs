macro_rules! deps {
    () => {
        Registry!();
        ServerResult!();
        ContextSelectionSet!();
        OutputType!();
        Field!();
    };
}

macro_rules! impl_895 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType + Sync > OutputType for Option < T > { fn type_name () -> Cow < 'static , str > { T :: type_name () } fn qualified_type_name () -> String { T :: type_name () . to_string () } fn create_type_info (registry : & mut registry :: Registry) -> String { T :: create_type_info (registry) ; T :: type_name () . to_string () } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { if let Some (inner) = self { match OutputType :: resolve (inner , ctx , field) . await { Ok (value) => Ok (value) , Err (err) => { ctx . add_error (err) ; Ok (Value :: Null) } } } else { Ok (Value :: Null) } } }
    };
}

impl_895!();