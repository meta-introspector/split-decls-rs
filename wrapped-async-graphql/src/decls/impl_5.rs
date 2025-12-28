macro_rules! deps {
    () => {
        Registry!();
        OutputType!();
        Result!();
        Error!();
        Field!();
        ServerResult!();
        ContextSelectionSet!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType + Sync , E : Into < Error > + Send + Sync + Clone > OutputType for Result < T , E > { fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut Registry) -> String { T :: create_type_info (registry) } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { match self { Ok (value) => value . resolve (ctx , field) . await , Err (err) => Err (ctx . set_error_path (err . clone () . into () . into_server_error (field . pos))) , } } }
    };
}

impl_5!();